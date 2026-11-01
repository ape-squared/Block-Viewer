//! Block login / loading stage background: deep surface sky, modular
//! drifting grid, stacked signal cubes, soft data blooms. No northern-light
//! curtains. Drawn with egui meshes (no textures).

use crate::theme::Palette;
use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{Color32, Pos2, Rect, Shape};

fn v(pos: Pos2, color: Color32) -> Vertex {
    Vertex { pos, uv: WHITE_UV, color }
}

fn glow(c: Color32, k: f32) -> Color32 {
    let k = k.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied((c.r() as f32 * k) as u8, (c.g() as f32 * k) as u8, (c.b() as f32 * k) as u8, 0)
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

fn hash(i: u32) -> f32 {
    let mut x = i.wrapping_mul(0x9E37_79B9) ^ 0x85EB_CA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    (x & 0xFFFF) as f32 / 65535.0
}

fn gradient(mesh: &mut Mesh, r: Rect, top: Color32, bottom: Color32) {
    let i = mesh.vertices.len() as u32;
    mesh.vertices.push(v(r.left_top(), top));
    mesh.vertices.push(v(r.right_top(), top));
    mesh.vertices.push(v(r.right_bottom(), bottom));
    mesh.vertices.push(v(r.left_bottom(), bottom));
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

fn quad(mesh: &mut Mesh, r: Rect, tl: Color32, tr: Color32, br: Color32, bl: Color32) {
    let i = mesh.vertices.len() as u32;
    mesh.vertices.push(v(r.left_top(), tl));
    mesh.vertices.push(v(r.right_top(), tr));
    mesh.vertices.push(v(r.right_bottom(), br));
    mesh.vertices.push(v(r.left_bottom(), bl));
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

/// Paint the background over `rect`; `t` = time in seconds, `alpha` fades it.
pub fn paint(painter: &egui::Painter, p: &Palette, rect: Rect, t: f32, alpha: f32) {
    painter.extend(shapes(p, rect, t, alpha, false));
}

/// Softened stage for login cards (no stars, softer grid).
pub fn frosted(p: &Palette, rect: Rect, t: f32) -> Vec<Shape> {
    shapes(p, rect, t, 1.0, true)
}

pub const LOGIN_TINT: f32 = 0.58;

pub fn frosted_pane(p: &Palette, screen: Rect, t: f32, pane: Rect, lights: Vec<Shape>) -> Vec<Shape> {
    let mut out = frosted(p, screen, t);
    out.extend(lights);
    let alpha = (f32::from(p.panel_alpha) * LOGIN_TINT) as u8;
    out.push(Shape::rect_filled(
        pane,
        0.0,
        Color32::from_rgba_unmultiplied(p.panel.r(), p.panel.g(), p.panel.b(), alpha),
    ));
    out
}

fn shapes(p: &Palette, rect: Rect, t: f32, alpha: f32, soft: bool) -> Vec<Shape> {
    let mut out = Vec::new();
    let a = alpha.clamp(0.0, 1.0);
    let (w, h) = (rect.width(), rect.height());
    let fade = |c: Color32| c.gamma_multiply(a);

    // flat Block surface sky
    let mut sky = Mesh::default();
    let top = mix(p.navy, Color32::BLACK, 0.35);
    let mid = p.navy;
    let bot = mix(p.navy, p.indigo, 0.12);
    let split = Rect::from_min_max(rect.min, Pos2::new(rect.right(), rect.top() + h * 0.55));
    gradient(&mut sky, split, fade(top), fade(mid));
    gradient(&mut sky, Rect::from_min_max(split.left_bottom(), rect.max), fade(mid), fade(bot));
    out.push(Shape::mesh(sky));

    // sparse signal nodes (not a starfield)
    if !soft {
        for i in 0..48u32 {
            let x = rect.left() + hash(i * 5) * w;
            let y = rect.top() + hash(i * 5 + 1) * h * 0.7;
            let tw = 0.45 + 0.55 * (t * (0.4 + hash(i) * 1.2) + hash(i + 9) * std::f32::consts::TAU).sin().abs();
            let col = if i % 3 == 0 { p.violet } else { p.indigo };
            out.push(Shape::circle_filled(Pos2::new(x, y), 1.1, glow(col, 0.35 * tw * a)));
        }
    }

    // soft brand blooms
    for (cx, cy, rad, col, inten) in [
        (0.18, 0.22, 0.48, p.violet, 0.11),
        (0.82, 0.30, 0.40, p.indigo, 0.10),
        (0.50, 0.58, 0.32, p.teal, 0.06),
    ] {
        out.push(radial_glow_shape(
            Pos2::new(rect.left() + cx * w, rect.top() + cy * h),
            rad * w.min(h),
            col,
            inten * a * if soft { 0.5 } else { 1.0 },
        ));
    }

    // primary modular grid
    {
        let mut grid = Mesh::default();
        let cell = (w.min(h) / 14.0).clamp(24.0, 54.0);
        let drift = if soft { t * 1.5 } else { t * 7.0 };
        let origin = Pos2::new(rect.left() + drift.rem_euclid(cell), rect.top() + (drift * 0.55).rem_euclid(cell));
        let line = |mesh: &mut Mesh, a: Pos2, b: Pos2, c: Color32, thick: f32| {
            let i = mesh.vertices.len() as u32;
            let d = b - a;
            let len = d.length().max(1.0);
            let n = Pos2::new(-d.y / len * thick, d.x / len * thick);
            mesh.vertices.push(v(Pos2::new(a.x + n.x, a.y + n.y), c));
            mesh.vertices.push(v(Pos2::new(a.x - n.x, a.y - n.y), c));
            mesh.vertices.push(v(Pos2::new(b.x - n.x, b.y - n.y), c));
            mesh.vertices.push(v(Pos2::new(b.x + n.x, b.y + n.y), c));
            mesh.add_triangle(i, i + 1, i + 2);
            mesh.add_triangle(i, i + 2, i + 3);
        };
        let thick = if soft { 0.8 } else { 1.35 };
        let gcol = glow(p.violet, (if soft { 0.035 } else { 0.09 }) * a);
        let mut x = origin.x;
        while x < rect.right() {
            line(&mut grid, Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom()), gcol, thick);
            x += cell;
        }
        let mut y = origin.y;
        while y < rect.bottom() {
            line(&mut grid, Pos2::new(rect.left(), y), Pos2::new(rect.right(), y), gcol, thick);
            y += cell;
        }

        // left-side stacked blocks (brand mark)
        let base = Pos2::new(rect.left() + cell * 0.9, rect.bottom() - cell * 2.4);
        for (ox, oy, k) in [(0.0, 0.0, 0.22), (0.6, -0.4, 0.14), (1.15, 0.1, 0.17), (0.35, -0.85, 0.10)] {
            let r = Rect::from_min_size(Pos2::new(base.x + ox * cell, base.y + oy * cell), egui::Vec2::splat(cell * 0.7));
            quad(
                &mut grid,
                r,
                glow(p.indigo, 0.0),
                glow(p.violet, k * 0.5 * a),
                glow(p.violet, k * a),
                glow(p.indigo, k * 0.35 * a),
            );
        }
        // right accent stack
        let base2 = Pos2::new(rect.right() - cell * 2.6, rect.bottom() - cell * 1.8);
        for (ox, oy, k) in [(0.0, 0.0, 0.16), (0.55, -0.3, 0.11)] {
            let r = Rect::from_min_size(Pos2::new(base2.x + ox * cell, base2.y + oy * cell), egui::Vec2::splat(cell * 0.65));
            quad(
                &mut grid,
                r,
                glow(p.teal, 0.0),
                glow(p.indigo, k * 0.45 * a),
                glow(p.violet, k * a),
                glow(p.indigo, k * 0.3 * a),
            );
        }
        out.push(Shape::mesh(grid));
    }

    // floor band
    let mut floor = Mesh::default();
    let fr = Rect::from_min_max(Pos2::new(rect.left(), rect.top() + h * 0.78), rect.max);
    gradient(
        &mut floor,
        fr,
        glow(p.violet, 0.0),
        Color32::from_rgba_unmultiplied(p.navy.r(), p.navy.g(), p.navy.b(), (40.0 * a) as u8),
    );
    out.push(Shape::mesh(floor));
    out
}

pub fn radial_glow(painter: &egui::Painter, center: Pos2, radius: f32, color: Color32, intensity: f32) {
    painter.add(radial_glow_shape(center, radius, color, intensity));
}

pub fn radial_glow_shape(center: Pos2, radius: f32, color: Color32, intensity: f32) -> Shape {
    let mut m = Mesh::default();
    const SEG: usize = 48;
    const RINGS: [(f32, f32); 4] = [(0.0, 1.0), (0.35, 0.62), (0.7, 0.22), (1.0, 0.0)];
    for (r, k) in RINGS {
        for s in 0..SEG {
            let a = s as f32 / SEG as f32 * std::f32::consts::TAU;
            let pos = center + egui::vec2(a.cos(), a.sin()) * radius * r;
            m.vertices.push(v(pos, glow(color, intensity * k)));
        }
    }
    for ring in 0..RINGS.len() as u32 - 1 {
        for s in 0..SEG as u32 {
            let (a0, a1) = (ring * SEG as u32 + s, ring * SEG as u32 + (s + 1) % SEG as u32);
            let (b0, b1) = (a0 + SEG as u32, a1 + SEG as u32);
            m.add_triangle(a0, b0, b1);
            m.add_triangle(a0, b1, a1);
        }
    }
    Shape::mesh(m)
}
