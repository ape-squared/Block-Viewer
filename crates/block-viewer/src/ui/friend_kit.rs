//! First-run checklist of day-one tools (People, AO, derender list, updates).

use super::widgets::{self, flat_button_sized, primary_button_sized};
use crate::theme::Palette;
use egui::{RichText, Vec2};

/// Actions the kit asks the app for when a row is clicked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KitAction {
    People,
    Ao,
    Blacklist,
    Updates,
    Dismiss,
}

/// Show the first-run friend kit. Returns an action when the user clicks.
pub fn show(ctx: &egui::Context, p: &Palette) -> Option<KitAction> {
    let mut action = None;
    let width = (ctx.content_rect().width() - 64.0).clamp(320.0, 480.0);
    widgets::Modal::new(egui::Id::unique("friend_kit")).show(ctx, |ui| {
        ui.set_width(width);
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
        ui.label(RichText::new(block_i18n::tr!("friend-kit-title")).size(22.0).strong().color(p.ink));
        ui.label(RichText::new(block_i18n::tr!("friend-kit-intro")).size(13.0).color(p.muted));
        ui.add_space(4.0);
        for (label, act) in [
            (block_i18n::tr!("friend-kit-people"), KitAction::People),
            (block_i18n::tr!("friend-kit-ao"), KitAction::Ao),
            (block_i18n::tr!("friend-kit-blacklist"), KitAction::Blacklist),
            (block_i18n::tr!("friend-kit-updates"), KitAction::Updates),
        ] {
            if flat_button_sized(ui, p, label, Vec2::new(width, 28.0)).clicked() {
                action = Some(act);
            }
        }
        ui.add_space(8.0);
        if primary_button_sized(ui, p, block_i18n::tr!("friend-kit-got-it"), Vec2::new(width, 34.0)).clicked() {
            action = Some(KitAction::Dismiss);
        }
    });
    action
}
