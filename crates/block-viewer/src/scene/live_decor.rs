//! LiveDecor: hologram where an inventory object will rez while dragging it
//! onto the world. Same loading-hologram look as the Create tool (#854), so
//! decor can be aimed before the simulator rezzes it.

use crate::build::shapes;
use crate::world::World;
use block_render::{DrawCmd, DrawLists, DrawRecord, MeshAlloc, Renderer, rez_mode};
use crossbeam_channel::{Receiver, TryRecvError};
use glam::{Mat4, Quat, Vec3};
use std::time::Instant;

/// Placement of the LiveDecor hologram on a rez surface.
#[derive(Debug, Clone, Copy)]
pub struct Placement {
    position: Vec3,
    scale: Vec3,
    bottom: f32,
    height: f32,
}

impl Placement {
    /// Box-sized ghost at `point` (surface hit from inventory drag).
    pub fn new(world: &World, point: Vec3, scale: Vec3) -> Option<Self> {
        if !point.is_finite() || !scale.is_finite() || scale.min_element() <= 0.0 {
            return None;
        }
        crate::build::BuildTool::region_at(world, point)?;
        let half_z = scale.z * 0.5;
        Some(Self {
            position: point + Vec3::Z * half_z,
            scale,
            bottom: point.z,
            height: scale.z,
        })
    }

    fn record(self, start: u32) -> DrawRecord {
        DrawRecord {
            model: Mat4::from_scale_rotation_translation(self.scale, Quat::IDENTITY, self.position).to_cols_array_2d(),
            base_color: [0.55, 0.72, 1.0, 1.0],
            rez: [start, rez_mode::HOLOGRAM | (3 << 20), self.bottom.to_bits(), self.height.to_bits()],
            ..DrawRecord::default()
        }
    }
}

type Prepared = Vec<super::jobs::FaceGeom>;

enum Geometry {
    Pending(Receiver<Prepared>),
    Ready(Vec<MeshAlloc>),
}

/// LiveDecor hologram state for inventory rez drag.
#[derive(Default)]
pub struct LiveDecor {
    geometry: Option<Geometry>,
    record: Option<u32>,
    start: Option<u32>,
    active: bool,
}

impl LiveDecor {
    pub fn update(&mut self, renderer: &mut Renderer, placement: Option<Placement>) {
        self.active = false;
        let Some(placement) = placement else {
            self.start = None;
            return;
        };
        let start = *self.start.get_or_insert_with(|| renderer.anim_clock().ms(Instant::now()));
        if self.geometry.is_none() {
            let params = shapes::shapes()[0].params();
            let (send, receive) = crossbeam_channel::bounded(1);
            rayon::spawn(move || {
                let mesh = block_prim::generate_volume(&params, block_prim::volume::DETAIL_SCALES[2]);
                let (faces, _, _) = super::jobs::volume_to_faces(&mesh);
                let _ = send.send(faces.into_iter().flatten().collect());
            });
            self.geometry = Some(Geometry::Pending(receive));
        }
        let ready = match self.geometry.as_mut() {
            Some(Geometry::Pending(receive)) => match receive.try_recv() {
                Ok(faces) => {
                    let meshes: Vec<_> = faces.iter().filter_map(|f| renderer.upload_mesh(&f.vertices, &f.indices)).collect();
                    if meshes.len() != faces.len() {
                        for mesh in meshes {
                            renderer.free_mesh(mesh);
                        }
                        log::warn!("LiveDecor: geometry arena full");
                        self.geometry = Some(Geometry::Ready(Vec::new()));
                        return;
                    }
                    self.geometry = Some(Geometry::Ready(meshes));
                    true
                }
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => {
                    log::warn!("LiveDecor: geometry preparation failed");
                    self.geometry = Some(Geometry::Ready(Vec::new()));
                    return;
                }
            },
            Some(Geometry::Ready(_)) => true,
            None => return,
        };
        if !ready {
            return;
        }
        let record = placement.record(start);
        if let Some(slot) = self.record {
            renderer.records.set(slot, record);
        } else {
            self.record = Some(renderer.records.alloc(record));
        }
        self.active = true;
    }

    pub fn append(&self, lists: &mut DrawLists) {
        if !self.active {
            return;
        }
        let Some(Geometry::Ready(meshes)) = self.geometry.as_ref() else {
            return;
        };
        let Some(record) = self.record else { return };
        lists.creation_preview.extend(meshes.iter().map(|m| DrawCmd {
            index_count: m.index_count,
            first_index: m.index_offset,
            base_vertex: m.vertex_offset as i32,
            record,
            bounds: [0.0; 4],
        }));
    }

    pub fn clear(&mut self, renderer: &mut Renderer) {
        if let Some(Geometry::Ready(meshes)) = self.geometry.take() {
            for mesh in meshes {
                renderer.free_mesh(mesh);
            }
        }
        if let Some(record) = self.record.take() {
            renderer.records.free(record);
        }
        self.active = false;
        self.start = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn live_decor_sits_on_the_surface_with_hologram_rez() {
        let mut world = World::new(Arc::new(super::super::avatar::AvatarLibrary::load()));
        for event in crate::demo::events() {
            world.apply(event);
        }
        let point = world.agent.position + Vec3::new(3.0, 2.0, 0.0);
        let placement = Placement::new(&world, point, Vec3::splat(0.5)).expect("demo surface");
        assert!((placement.bottom - point.z).abs() < 1e-4);
        assert!((placement.height - 0.5).abs() < 1e-4);
        let record = placement.record(42);
        assert_eq!(record.rez[0], 42);
        assert_eq!(record.rez[1] & 0xff, rez_mode::HOLOGRAM);
        assert!(Placement::new(&world, Vec3::NAN, Vec3::ONE).is_none());
    }
}
