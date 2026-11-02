//! Login sidebar: start destinations (Legacy-style), without Aurora news.

use crate::login_favorites::Entry as FavoritePlace;
use crate::theme::Palette;
use crate::ui::{icons::Icons, widgets};
use egui::{RichText, Sense, Vec2};

/// Destinations shown as one-click start choices (Legacy-style).
const DESTINATIONS: &[(&str, &str, &str)] = &[
    ("home", "home-filled", "common-home"),
    ("last", "history", "ui-login-last-location"),
    ("region", "map-pin-filled", "ui-login-region"),
];

/// Right-hand login card: start destinations (replaces Aurora news).
pub fn panel(
    ui: &mut egui::Ui,
    p: &Palette,
    icons: &Icons,
    start_location: &mut String,
    start_slurl: &mut Option<crate::slurl::Location>,
    favorites: &[FavoritePlace],
    height: f32,
    on_region: &mut dyn FnMut(),
) {
    const MARGIN: i8 = 16;
    super::card(ui, p, MARGIN, false, |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(height - 2.0 * f32::from(MARGIN));
        widgets::section_header(ui, p, "map-pin-filled", block_i18n::tr!("ui-login-destinations"));
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
                    let selected = start_slurl.is_none() && start_location.as_str() == value;
                    if dest_row(ui, p, icons, icon, &label, selected) {
                        *start_location = value.into();
                        *start_slurl = None;
                        if value == "region" {
                            on_region();
                        }
                    }
                }
                if !favorites.is_empty() {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(block_i18n::tr!("ui-login-favorites"))
                            .size(12.0)
                            .strong()
                            .color(p.muted),
                    );
                    ui.add_space(4.0);
                    for fav in favorites {
                        let loc = crate::slurl::parse(&fav.place);
                        let selected = loc.as_ref().is_some_and(|l| start_slurl.as_ref() == Some(l));
                        if dest_row(ui, p, icons, "star-filled", &fav.name, selected) {
                            *start_location = "favorite".into();
                            *start_slurl = loc;
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

fn dest_row(ui: &mut egui::Ui, p: &Palette, icons: &Icons, icon: &str, label: &str, selected: bool) -> bool {
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
    resp.clicked()
}
