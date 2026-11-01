//! Login sidebar: destinations and a short Block message instead of Aurora news.
//! Legacy Viewer shows grid MOTD / destinations beside the login form; keep that
//! useful surface without shipping Aurora release notes.

use crate::theme::Palette;
use crate::ui::{icons::Icons, widgets};
use egui::{RichText, Sense, Vec2};

/// Destinations shown as one-click start choices (Legacy-style).
const DESTINATIONS: &[(&str, &str, &str)] = &[
    ("home", "home-filled", "common-home"),
    ("last", "history", "ui-login-last-location"),
    ("region", "map-pin-filled", "ui-login-region"),
];

/// Welcome / MOTD line for Block closed testing (not grid Halloween MOTD).
pub fn block_test_message() -> &'static str {
    "Block Viewer closed test — thank you for helping. Log in, try pie menus, nameplates, and teleport; report anything that breaks."
}

/// Right-hand login card: message + start destinations (replaces Aurora news).
pub fn panel(ui: &mut egui::Ui, p: &Palette, icons: &Icons, start_location: &mut String, height: f32, on_region: &mut dyn FnMut()) {
    const MARGIN: i8 = 16;
    super::card(ui, p, MARGIN, false, |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(height - 2.0 * f32::from(MARGIN));
        widgets::section_header(ui, p, "map-pin-filled", block_i18n::tr!("ui-login-destinations"));
        ui.add_space(6.0);
        ui.add(egui::Label::new(RichText::new(block_i18n::tr!("ui-login-block-test-motd")).size(12.5).color(p.ink)).wrap());
        ui.add_space(4.0);
        ui.label(RichText::new(block_test_message()).size(12.0).color(p.muted));
        ui.add_space(10.0);
        widgets::divider(ui, p);
        ui.add_space(8.0);
        ui.label(RichText::new(block_i18n::tr!("ui-login-start")).size(12.0).strong().color(p.muted));
        ui.add_space(6.0);
        let rest = ui.available_height();
        egui::ScrollArea::vertical()
            .max_height(rest)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                for &(value, icon, label_key) in DESTINATIONS {
                    let label = match label_key {
                        "common-home" => block_i18n::tr!("common-home"),
                        "ui-login-last-location" => block_i18n::tr!("ui-login-last-location"),
                        "ui-login-region" => block_i18n::tr!("ui-login-region"),
                        _ => label_key,
                    };
                    let selected = start_location.as_str() == value;
                    let fill = if selected { p.violet.gamma_multiply(0.35) } else { p.raised };
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 36.0), Sense::click());
                    let radius = ui.visuals().widgets.inactive.corner_radius;
                    ui.painter().rect_filled(rect, radius, fill);
                    ui.painter().rect_stroke(
                        rect,
                        radius,
                        egui::Stroke::new(1.0, if selected { p.violet_light } else { p.raised }),
                        egui::StrokeKind::Inside,
                    );
                    if let Some(tex) = icons.get(icon).cloned().or_else(|| crate::ui::icons::global(icon)) {
                        let icon_r = egui::Rect::from_center_size(egui::pos2(rect.left() + 18.0, rect.center().y), Vec2::splat(16.0));
                        ui.painter().image(
                            tex.id(),
                            icon_r,
                            egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                            if selected { p.violet_light } else { p.muted },
                        );
                    }
                    ui.painter().text(
                        egui::pos2(rect.left() + 36.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        egui::FontId::proportional(13.0),
                        p.ink,
                    );
                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if resp.clicked() {
                        *start_location = value.into();
                        if value == "region" {
                            on_region();
                        }
                    }
                }
                ui.add_space(8.0);
                ui.label(
                    RichText::new(block_i18n::tr!("ui-login-destinations-hint"))
                        .size(11.5)
                        .color(p.muted_dim),
                );
            });
    });
}
