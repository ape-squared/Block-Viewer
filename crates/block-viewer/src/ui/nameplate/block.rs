//! Block Dev mark on a team plate (`world::staff`): stacked signal/data
//! cubes rising behind the top edge, a border whose colors flow around it
//! over a soft halo, and on the full and compact plates the "Block Dev"
//! tag over its top left edge. Built every frame for the few plates that
//! have it.

use crate::theme::Palette;
use egui::{Color32, Mesh, Pos2, Rect, Vec2};
use std::f32::consts::{PI, TAU};

/// Height of the Block cube stack over the plate's height.
const CURTAIN_HEIGHT: f32 = 0.95;
/// Premultiplied cover for additive cube faces.
const CURTAIN_COVER: f32 = 0.62;
/// The halo around the border, in mockup pixels, and its brightness.
const HALO: f32 = 7.0;
const HALO_LIGHT: f32 = 0.42;

/// The tag, in mockup pixels: its height, padding, icon, the gap between
/// the icon and the text, and the text's size.
pub const TAG_HEIGHT: f32 = 17.0;
pub const TAG_PAD: f32 = 7.0;
pub const TAG_ICON: f32 = 11.0;
pub const TAG_GAP: f32 = 4.0;
pub const TAG_TEXT: f32 = 10.5;
/// How far the tag's middle stands above the plate's top edge: it floats
/// mostly outside, clear of the name.
pub const TAG_RISE: f32 = 3.0;

/// Squarer corner radius than a stadium pill (kept local: nameplate mesh
/// geometry no longer exports this helper after the Aurora sync).
fn plate_corner(rect: Rect) -> f32 {
    (rect.height() * 0.18).clamp(2.5, 6.0).min(rect.width() * 0.4)
}

/// Width of the tag around a text `text` points wide, at `scale`.
pub fn tag_width(text: f32, scale: f32) -> f32 {
    (2.0 * TAG_PAD + TAG_ICON + TAG_GAP) * scale + text
}

/// Seconds of the light's motion at `now`; held still with "Reduce
/// animations".
pub fn time(now: f64, still: bool) -> f32 {
    if still { 7.0 } else { now.rem_euclid(3600.0) as f32 }
}

/// A light of `color` at `i` (0..1): mostly added to what is behind it.
pub(super) fn light(color: Color32, i: f32, cover: f32) -> Color32 {
    let i = i.clamp(0.0, 1.0);
    let [r, g, b, _] = color.to_opaque().to_array();
    let k = |c: u8| (c as f32 * i).round() as u8;
    Color32::from_rgba_premultiplied(k(r), k(g), k(b), (255.0 * i * cover).round() as u8)
}

fn smooth(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// Border color around a Block Dev plate: signal ↔ data, with a running glint.
pub fn border_color(p: &Palette, s: f32, t: f32) -> Color32 {
    let u = 0.5 + 0.5 * (TAU * (2.0 * s - 0.09 * t)).sin();
    let glint = 0.82 + 0.18 * (TAU * (5.0 * s + 0.24 * t)).sin();
    p.violet.lerp_to_gamma(p.indigo, u).to_opaque().gamma_multiply(glint)
}

/// Stacked Block cubes rising behind a team plate (replaces aurora curtains).
pub fn curtains(mesh: &mut Mesh, p: &Palette, rect: Rect, ppp: f32, t: f32, alpha: f32) {
    let _ = ppp;
    let h = rect.height().max(1.0);
    let full = h * CURTAIN_HEIGHT;
    let cell = (h * 0.42).clamp(8.0, 18.0);
    let base_y = rect.center().y - h * 0.15;
    // three staggered columns of cubes across the plate width
    let cols = [(0.18, 0.0, 3usize), (0.50, 0.35, 4usize), (0.82, 0.15, 3usize)];
    for (cx, phase, n) in cols {
        let x0 = rect.left() + cx * rect.width() - cell * 0.5;
        for i in 0..n {
            let bob = (t * 1.4 + phase + i as f32 * 0.7).sin() * cell * 0.08;
            let y1 = base_y - (i as f32 + 1.0) * cell * 0.92 - bob;
            let y0 = y1 + cell * 0.78;
            let fade = smooth(1.0 - i as f32 / n as f32);
            let pulse = 0.55 + 0.45 * (0.5 + 0.5 * (t * 2.0 + phase + i as f32).sin());
            let inten = fade * pulse * alpha;
            let fill = if i % 2 == 0 { p.violet } else { p.indigo };
            let r = Rect::from_min_max(Pos2::new(x0, y1), Pos2::new(x0 + cell, y0.min(base_y)));
            if r.height() < 1.0 {
                continue;
            }
            let first = mesh.vertices.len() as u32;
            mesh.colored_vertex(r.left_top(), light(fill, inten * 0.15, CURTAIN_COVER));
            mesh.colored_vertex(r.right_top(), light(fill, inten * 0.55, CURTAIN_COVER));
            mesh.colored_vertex(r.right_bottom(), light(fill, inten, CURTAIN_COVER));
            mesh.colored_vertex(r.left_bottom(), light(p.indigo, inten * 0.35, CURTAIN_COVER));
            mesh.indices
                .extend_from_slice(&[first, first + 1, first + 2, first, first + 2, first + 3]);
            // soft glow above the top of the stack
            if i + 1 == n {
                let tip = Pos2::new(r.center().x, r.top() - full * 0.08);
                let g0 = mesh.vertices.len() as u32;
                mesh.colored_vertex(tip, light(p.violet_light, inten * 0.5, 0.35));
                mesh.colored_vertex(r.left_top(), light(p.violet, 0.0, 0.35));
                mesh.colored_vertex(r.right_top(), light(p.indigo, 0.0, 0.35));
                mesh.indices.extend_from_slice(&[g0, g0 + 1, g0 + 2]);
            }
        }
    }
}

/// Points around a rounded rectangle, clockwise from the left end of its
/// top edge, a few pixels apart even along its straight edges (the colors
/// flow along them): position, outward normal, and how far around (0..1).
struct Perimeter {
    points: Vec<(Pos2, Vec2, f32)>,
    center: Pos2,
}

impl Perimeter {
    /// Name-plate outline: squarer corners than a stadium pill.
    fn of(rect: Rect, ppp: f32) -> Perimeter {
        Perimeter::rounded(rect, plate_corner(rect), ppp)
    }

    fn rounded(rect: Rect, radius: f32, ppp: f32) -> Perimeter {
        let r = radius.clamp(0.0, rect.height().min(rect.width()) * 0.5);
        let (w, h) = ((rect.width() - 2.0 * r).max(0.0), (rect.height() - 2.0 * r).max(0.0));
        let (l, t) = (rect.left() + r, rect.top() + r);
        let arc = ((0.5 * PI * r * ppp / 3.0).ceil() as usize).clamp(2, 16);
        let mut points = Vec::new();
        // each side then its corner: (start, direction, length, outward normal, corner center)
        let sides = [
            (Pos2::new(l, rect.top()), Vec2::X, w, -Vec2::Y, Pos2::new(l + w, t)),
            (Pos2::new(rect.right(), t), Vec2::Y, h, Vec2::X, Pos2::new(l + w, t + h)),
            (Pos2::new(l + w, rect.bottom()), -Vec2::X, w, Vec2::Y, Pos2::new(l, t + h)),
            (Pos2::new(rect.left(), t + h), -Vec2::Y, h, -Vec2::X, Pos2::new(l, t)),
        ];
        for (k, (start, dir, length, normal, corner)) in sides.into_iter().enumerate() {
            let steps = if length > 0.01 {
                ((length * ppp / 4.0).ceil() as usize).max(1)
            } else {
                0
            };
            for i in 0..steps {
                points.push((start + dir * (length * i as f32 / steps as f32), normal));
            }
            if r > 0.0 {
                let from = -PI * 0.5 + k as f32 * PI * 0.5;
                for i in 0..arc {
                    let a = from + 0.5 * PI * i as f32 / arc as f32;
                    let n = Vec2::new(a.cos(), a.sin());
                    points.push((corner + n * r, n));
                }
            }
        }
        let total = (2.0 * (w + h) + TAU * r).max(f32::EPSILON);
        let mut along = 0.0;
        let mut last = points[0].0;
        let points = points
            .into_iter()
            .map(|(pos, n)| {
                along += (pos - last).length();
                last = pos;
                (pos, n, along / total)
            })
            .collect();
        Perimeter {
            points,
            center: rect.center(),
        }
    }

    /// A ring `d` outside the pill (inside when negative), each vertex in
    /// the color `color` gives for how far around it is.
    fn ring(&self, mesh: &mut Mesh, d: f32, mut color: impl FnMut(f32) -> Color32) -> u32 {
        let first = mesh.vertices.len() as u32;
        for &(pos, n, s) in &self.points {
            mesh.colored_vertex(pos + n * d, color(s));
        }
        first
    }

    fn band(&self, mesh: &mut Mesh, inner: u32, outer: u32) {
        let n = self.points.len() as u32;
        for i in 0..n {
            let j = (i + 1) % n;
            mesh.indices
                .extend_from_slice(&[inner + i, inner + j, outer + j, inner + i, outer + j, outer + i]);
        }
    }

    fn fill(&self, mesh: &mut Mesh, ring: u32, color: Color32) {
        let center = mesh.vertices.len() as u32;
        mesh.colored_vertex(self.center, color);
        let n = self.points.len() as u32;
        for i in 0..n {
            mesh.indices.extend_from_slice(&[center, ring + i, ring + (i + 1) % n]);
        }
    }
}

/// A plate's border, `width` points along the inside of its edge, in the
/// color `edge` gives for each point around it (0..1), over a halo of the
/// same colors `scale` mockup pixels to points.
pub(super) fn border(mesh: &mut Mesh, rect: Rect, ppp: f32, alpha: f32, width: f32, scale: f32, edge: impl Fn(f32) -> Color32) {
    let o = Perimeter::of(rect, ppp);
    let f = 0.5 / ppp;
    let color = |s: f32| edge(s).gamma_multiply(alpha);
    let halo = |s: f32| light(edge(s), HALO_LIGHT * alpha, 0.5);
    let a = o.ring(mesh, -width - f, |_| Color32::TRANSPARENT);
    let b = o.ring(mesh, -width + f, color);
    let c = o.ring(mesh, -f, color);
    let d = o.ring(mesh, f, halo);
    let e = o.ring(mesh, HALO * scale, |_| Color32::TRANSPARENT);
    o.band(mesh, a, b);
    o.band(mesh, b, c);
    o.band(mesh, c, d);
    o.band(mesh, d, e);
}

/// The tag at `rect`, with corners of `radius` (half its height on a
/// plate): `fill` inside a border `width` wide, in the color `color` gives
/// for each point around it (0..1).
#[allow(clippy::too_many_arguments)]
pub(super) fn tag(
    mesh: &mut Mesh,
    rect: Rect,
    radius: f32,
    ppp: f32,
    fill: Color32,
    alpha: f32,
    width: f32,
    color: impl Fn(f32) -> Color32,
) {
    let o = Perimeter::rounded(rect, radius, ppp);
    let f = 0.5 / ppp;
    let fill = fill.gamma_multiply(alpha);
    let edge = |s: f32| super::mesh::over(color(s).gamma_multiply(alpha), fill);
    let inner = o.ring(mesh, -width - f, |_| fill);
    o.fill(mesh, inner, fill);
    let b = o.ring(mesh, -width + f, edge);
    let c = o.ring(mesh, -f, edge);
    let d = o.ring(mesh, f, |_| Color32::TRANSPARENT);
    o.band(mesh, inner, b);
    o.band(mesh, b, c);
    o.band(mesh, c, d);
}

/// Tag letter color: signal → data across the word, with a running shine.
pub fn letter_color(p: &Palette, i: usize, n: usize, t: f32) -> Color32 {
    let u = i as f32 / n.saturating_sub(1).max(1) as f32;
    let shine = (TAU * (u * 0.8 - 0.25 * t)).sin().max(0.0).powi(3);
    p.violet
        .lerp_to_gamma(p.indigo, 0.15 + 0.7 * u)
        .lerp_to_gamma(p.ink, 0.2 + 0.45 * shine)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pill() -> Rect {
        Rect::from_min_size(Pos2::new(100.0, 50.0), Vec2::new(160.0, 50.0))
    }

    #[test]
    fn the_perimeter_goes_once_around_the_plate() {
        let rect = pill();
        let o = Perimeter::of(rect, 1.0);
        assert!(o.points.len() > 40);
        let r = plate_corner(rect);
        for &(pos, n, s) in &o.points {
            assert!((0.0..=1.0).contains(&s));
            assert!((n.length() - 1.0).abs() < 1e-4);
            let half = rect.size() * 0.5 - Vec2::splat(r);
            let d = (pos - rect.center()).abs() - half;
            let dist = d.x.max(d.y).min(0.0) + Vec2::new(d.x.max(0.0), d.y.max(0.0)).length() - r;
            assert!(dist.abs() < 0.05, "{pos:?} dist={dist}");
        }
        assert!(o.points.windows(2).all(|w| w[1].2 > w[0].2));
        // the straight edges are cut too: the colors flow along them
        assert!(o.points.iter().filter(|(pos, ..)| (pos.y - rect.top()).abs() < 0.01).count() > 10);
    }

    #[test]
    fn the_light_stays_inside_its_meshes() {
        let p = crate::theme::Theme::default().palette();
        let mut mesh = Mesh::default();
        curtains(&mut mesh, &p, pill(), 1.0, 3.0, 1.0);
        assert!(mesh.is_valid() && !mesh.indices.is_empty());
        // above the plate, never below its top edge
        assert!(
            mesh.vertices
                .iter()
                .all(|v| v.pos.y <= 76.0 && v.pos.y >= 50.0 - 50.0 * CURTAIN_HEIGHT * 1.31)
        );
        // three cube columns across the plate, staying inside its width
        let xs: Vec<_> = mesh.vertices.iter().map(|v| v.pos.x).collect();
        assert!(xs.iter().any(|&x| x < 140.0) && xs.iter().any(|&x| x > 220.0));
        assert!(xs.iter().all(|&x| (100.0..260.0).contains(&x)));
        // the tops are dark, the feet lit somewhere
        assert!(mesh.vertices.iter().any(|v| v.color.g() > 40));
        let mut ring = Mesh::default();
        let edge = |s: f32| border_color(&p, s, 3.0);
        border(&mut ring, pill(), 1.0, 1.0, 1.5, 0.76, edge);
        tag(&mut ring, pill(), 25.0, 2.0, p.navy, 0.9, 1.0, edge);
        tag(&mut ring, pill(), 3.0, 1.5, p.navy, 1.0, 1.0, edge);
        assert!(ring.is_valid());
        // the theme's small corners: every point within the rectangle's edge
        let o = Perimeter::rounded(pill(), 3.0, 1.0);
        assert!(o.points.iter().all(|(pos, ..)| pill().expand(0.01).contains(*pos)));
        assert!(o.points.iter().any(|(pos, ..)| pos.x == 260.0));
        // faded out: nothing
        let mut faded = Mesh::default();
        curtains(&mut faded, &p, pill(), 1.0, 3.0, 0.0);
        assert!(faded.vertices.iter().all(|v| v.color == Color32::TRANSPARENT));
    }

    #[test]
    fn the_light_moves_unless_held_still() {
        let p = crate::theme::Theme::default().palette();
        assert_ne!(border_color(&p, 0.1, 0.0), border_color(&p, 0.1, 2.0));
        assert_eq!(time(10.0, true), time(500.0, true));
        assert!(time(10.0, false) < time(11.0, false));
        // the word goes from green to violet
        assert_ne!(letter_color(&p, 0, 12, 0.0), letter_color(&p, 11, 12, 0.0));
        assert!(tag_width(40.0, 1.0) > 40.0);
    }
}
