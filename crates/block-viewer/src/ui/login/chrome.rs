//! Shared login information, community links, version footer and the
//! preferences button.
use super::{LoginAction, LoginForm};
use crate::theme::Palette;
use crate::ui::{
    icons::Icons,
    news::LoginInfo,
    widgets::{self, ActionKind},
};
use egui::{Context, Id, Rect, RichText, Vec2};

pub fn sidebar(
    ctx: &Context,
    p: &Palette,
    icons: &Icons,
    info: &LoginInfo,
    card: Rect,
    width: f32,
    lowest: f32,
    start_location: &mut String,
    start_slurl: &mut Option<crate::slurl::Location>,
    favorites: &[crate::login_favorites::Entry],
) -> bool {
    // Aurora right column: What's New above Destinations (Layout keeps the
    // centred account strip clear of this gutter).
    let column_x = ctx.content_rect().right() - 32.0 - width;
    let mut check_updates = false;
    let room = (lowest - card.top()).max(160.0);
    let news_h = (room * 0.42).clamp(120.0, 200.0);
    let gap = 12.0;
    let dest_h = (room - news_h - gap).max(120.0);
    egui::Area::new(Id::unique("login_whats_new"))
        .fixed_pos(egui::pos2(column_x, card.top()))
        .show(ctx, |ui| {
            ui.set_width(width);
            ui.set_max_width(width);
            check_updates = super::whats_new_panel(ui, p, icons, info, news_h);
        });
    egui::Area::new(Id::unique("login_destinations"))
        .fixed_pos(egui::pos2(column_x, card.top() + news_h + gap))
        .show(ctx, |ui| {
            ui.set_width(width);
            ui.set_max_width(width);
            super::destinations::panel(ui, p, icons, start_location, start_slurl, favorites, dest_h, &mut || {});
        });
    check_updates
}

fn version(ui: &mut egui::Ui, p: &Palette) {
    let label = block_i18n::tr!("ui-login-block-viewer-v", value0 = block_version::version().to_string());
    ui.label(RichText::new(label).size(11.5).color(p.muted_dim));
}

pub fn footer(ctx: &Context, p: &Palette, icons: &Icons, info: &LoginInfo, sidebar_visible: bool, form: &mut LoginForm) -> LoginAction {
    let mut action = LoginAction::None;
    let screen = ctx.content_rect();
    if form.safe_graphics.is_some() || form.photo_quality_off || form.camera_limits_restored || form.driver_notice.is_some() {
        super::driver_notice::column(ctx, |ui, width| {
            if let Some(state) = form.safe_graphics
                && let Some(choice) = super::safe_graphics_notice::card(ui, p, icons, state, width)
            {
                action = LoginAction::SafeGraphics(choice);
            }
            if form.photo_quality_off
                && let Some(back_on) = super::photo_quality_notice::card(ui, p, icons, width)
            {
                action = LoginAction::PhotoQualityOff(back_on);
            }
            if form.camera_limits_restored
                && let Some(open) = super::camera_limits_notice::card(ui, p, icons, width)
            {
                action = LoginAction::CameraLimits(open);
            }
            if let Some(notice) = &form.driver_notice
                && super::driver_notice::card(ui, p, icons, notice, width)
            {
                action = LoginAction::DismissDriverNotice;
            }
        });
    }
    // Claude mockup footer: version + GitHub first, community links secondary.
    egui::Area::new(Id::unique("login_links"))
        .anchor(egui::Align2::LEFT_BOTTOM, Vec2::new(25.0, -20.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                ui.label(
                    RichText::new(format!("BLOCK VIEWER {}", block_version::version()))
                        .size(11.5)
                        .color(p.muted),
                );
                super::link_button(ui, p, "brand-github-filled", "GitHub", crate::links::GITHUB);
                super::link_button(ui, p, "brand-discord-filled", "Discord", crate::links::DISCORD);
                let bug = widgets::action_button(
                    ui,
                    p,
                    "bug-filled",
                    block_i18n::tr!("bug-title"),
                    ActionKind::Caution,
                    Vec2::new(0.0, 30.0),
                );
                if bug.clicked() {
                    action = LoginAction::ReportBug;
                } else if bug.secondary_clicked() {
                    action = LoginAction::CopyVersion;
                }
                if !sidebar_visible
                    && widgets::action_button(
                        ui,
                        p,
                        "article-filled",
                        block_i18n::tr!("ui-login-news"),
                        ActionKind::Flat,
                        Vec2::new(0.0, 30.0),
                    )
                    .clicked()
                {
                    form.show_info = true;
                }
            });
            ui.add_space(4.0);
            ui.label(
                RichText::new(block_i18n::tr!("ui-login-independent-disclaimer"))
                    .size(10.5)
                    .color(p.muted_dim),
            );
        });
    if screen.width() < 1100.0 {
        egui::Area::new(Id::unique("login_footer_version"))
            .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-25.0, -59.0))
            .show(ctx, |ui| version(ui, p));
    }
    // The preferences before login (Firestorm opens them from the menu bar of
    // its login screen, menu_login.xml): one cogwheel in the top right corner.
    egui::Area::new(Id::unique("login_settings"))
        .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-25.0, 20.0))
        .show(ctx, |ui| {
            if widgets::action_button(ui, p, "settings-filled", "", ActionKind::Flat, Vec2::splat(30.0))
                .on_hover_text(block_i18n::tr!("common-preferences"))
                .clicked()
            {
                action = LoginAction::Preferences;
            }
        });
    // Left of it, the way to a larger, more readable interface before the
    // first login: Preferences › Accessibility.
    egui::Area::new(Id::unique("login_accessibility"))
        .anchor(egui::Align2::RIGHT_TOP, Vec2::new(-63.0, 20.0))
        .show(ctx, |ui| {
            if widgets::action_button(ui, p, "accessible-filled", "", ActionKind::Flat, Vec2::splat(30.0))
                .on_hover_text(block_i18n::tr!("login-accessibility"))
                .clicked()
            {
                action = LoginAction::Accessibility;
            }
        });
    egui::Area::new(Id::unique("login_quit"))
        .anchor(egui::Align2::RIGHT_BOTTOM, Vec2::new(-25.0, -20.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 14.0;
                if screen.width() >= 1100.0 {
                    version(ui, p);
                }
                if widgets::action_button(
                    ui,
                    p,
                    "power",
                    block_i18n::tr!("common-quit"),
                    ActionKind::FlatDanger,
                    Vec2::new(0.0, 30.0),
                )
                .clicked()
                {
                    action = LoginAction::Quit;
                }
            });
        });
    if form.show_info {
        let response = widgets::Modal::new(Id::unique("accounts_news_dialog")).show(ctx, |ui| {
            ui.set_width(340.0);
            if super::version_panel(ui, p, icons, info) {
                action = LoginAction::CheckUpdates;
            }
            ui.add_space(8.0);
            super::news_panel(ui, p, icons, info, 235.0);
            if widgets::default_button(ui.button(block_i18n::tr!("common-close"))).clicked() {
                form.show_info = false;
            }
        });
        if response.should_close() {
            form.show_info = false;
        }
    }
    action
}
