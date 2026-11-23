//! Preferences › Block: what Block does differently from the Legacy Viewer,
//! and for each redrawn element the way back to the Legacy look. The sidebar
//! entry uses the Block signal accent (docs/BRANDING.md).

use super::super::cx::{Cx, F, field};
use crate::build::MoveHologram;
use crate::settings::{ChatStyle, DestinationsStyle, NotificationsStyle, PhotoWindowStyle, Settings, WorldMapStyle};
use crate::theme::Palette;
use crate::world::beams::Style as BeamStyle;
use crate::world::lookat::{Aim, Style};
use egui::{Color32, FontId, Pos2, Rect, Sense, Vec2};

/// The two looks a redrawn element can take.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Look {
    Block,
    Legacy,
}

/// Every choice of the page set to one look (the colored sky ambient
/// included), and Block's water with it;
/// the avatar plates' own settings and the filing of received items are
/// left as they are, and so is a gaze on the point under the cursor: the
/// Block look does not choose to show others what the cursor is over.
/// Nor does it turn on the Instant movement, still experimental: only its
/// own row does; the Legacy look turns it off.
pub fn set_all(s: &mut Settings, look: Look) {
    let aurora = look == Look::Block;
    s.chat_style = if aurora { ChatStyle::Block } else { ChatStyle::Legacy };
    s.classic_name_tags = !aurora;
    s.lookat.style = if aurora { Style::Block } else { Style::Legacy };
    s.beams.style = if aurora { BeamStyle::Block } else { BeamStyle::Legacy };
    if !aurora {
        s.lookat.aim = Aim::Direction;
    }
    s.notifications_style = if aurora {
        NotificationsStyle::Block
    } else {
        NotificationsStyle::Legacy
    };
    s.world_map_style = if aurora { WorldMapStyle::Block } else { WorldMapStyle::Legacy };
    s.classic_bars = !aurora;
    s.classic_people = !aurora;
    s.classic_profiles = !aurora;
    s.photo_window = if aurora {
        PhotoWindowStyle::Block
    } else {
        PhotoWindowStyle::Legacy
    };
    s.classic_about_land = !aurora;
    s.classic_search = !aurora;
    s.classic_region_estate = !aurora;
    s.classic_inventory = !aurora;
    s.classic_outfit_gallery = !aurora;
    s.build.legacy_window = !aurora;
    s.classic_places = !aurora;
    s.destinations_style = if aurora {
        DestinationsStyle::Block
    } else {
        DestinationsStyle::Legacy
    };
    s.classic_pay_window = !aurora;
    s.legacy_money_tracker = !aurora;
    s.classic_controls = !aurora;
    s.classic_restart_alert = !aurora;
    s.classic_chat_lines = !aurora;
    s.classic_rez_effect = !aurora;
    s.classic_hover_tips = !aurora;
    if !aurora {
        s.predict_own_motion = false;
    }
    s.water_effects.enabled = aurora;
    s.synchronize_animations = aurora;
    s.colored_sky_ambient = aurora;
}

/// Block brand gradient at `t` (0..1): data cyan → signal lime (see theme).
pub(crate) fn block_color(p: &Palette, t: f32) -> Color32 {
    let stops = p.block_gradient();
    let x = t.clamp(0.0, 1.0) * (stops.len() - 1) as f32;
    let i = (x.floor() as usize).min(stops.len() - 2);
    stops[i].lerp_to_gamma(stops[i + 1], x - i as f32)
}

/// `text` in the aurora gradient, one letter after the other: the mark of
/// what is Block's own (this page's entry, the Weather titles).
pub(crate) fn block_text(p: &Palette, text: &str, font: FontId) -> egui::text::LayoutJob {
    let count = text.chars().count().max(2) as f32 - 1.0;
    let mut job = egui::text::LayoutJob::default();
    for (i, ch) in text.chars().enumerate() {
        let color = block_color(p, i as f32 / count);
        job.append(&ch.to_string(), 0.0, egui::TextFormat::simple(font.clone(), color));
    }
    job
}

/// Highlights the product name "Block" in the brand accent; rest in `ink`.
pub(crate) fn block_word(p: &Palette, text: &str, font: FontId, ink: Color32) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let mut rest = text;
    while let Some(at) = rest.find("Block") {
        job.append(&rest[..at], 0.0, egui::TextFormat::simple(font.clone(), ink));
        for (i, ch) in "Block".chars().enumerate() {
            let color = block_color(p, i as f32 / 4.0);
            job.append(&ch.to_string(), 0.0, egui::TextFormat::simple(font.clone(), color));
        }
        rest = &rest[at + "Block".len()..];
    }
    job.append(rest, 0.0, egui::TextFormat::simple(font, ink));
    job
}

/// The entry of the page in the sidebar: the neighbours' entry, with the
/// aurora gradient in its edge mark and its name.
pub fn nav_entry(ui: &mut egui::Ui, p: &Palette, label: &str, icon: &str, selected: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::click());
    let radius = ui.visuals().widgets.inactive.corner_radius;
    let face = if selected {
        p.violet.gamma_multiply(0.24)
    } else if response.hovered() {
        p.raised
    } else {
        p.raised.gamma_multiply(0.3)
    };
    ui.painter().rect_filled(rect, radius, face);
    let mark = Rect::from_min_max(rect.left_top(), Pos2::new(rect.left() + 3.0, rect.bottom()));
    ui.painter()
        .rect_filled(mark, 0.0, if selected { p.violet } else { p.violet.gamma_multiply(0.55) });
    if let Some(texture) = crate::ui::icons::global(icon) {
        ui.painter().image(
            texture.id(),
            Rect::from_center_size(rect.left_center() + Vec2::new(17.0, 0.0), Vec2::splat(17.0)),
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            p.teal,
        );
    }
    let font = FontId::new(12.0, crate::ui::fonts::semibold(ui.ctx()));
    let job = if selected {
        block_text(p, label, font)
    } else {
        let mut job = egui::text::LayoutJob::default();
        job.append(label, 0.0, egui::TextFormat::simple(font, p.ink));
        job
    };
    let galley = ui.painter().layout_job(job);
    ui.painter()
        .galley(rect.left_center() + Vec2::new(34.0, -galley.size().y / 2.0), galley, p.ink);
    response.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, selected, label));
    response.on_hover_text(label)
}

/// A choice between Block's look (first, in the accent) and the Legacy
/// one, over a "classic" switch.
fn look(cx: &mut Cx, name: &str, help: &str, aurora: &str, legacy: &str, f: &F<bool>) {
    cx.wide().choice(name, help, f, &[(false, aurora, true), (true, legacy, false)]);
}

/// An experimental choice: its warning joins its help.
fn warned(help: &str, warning: &str) -> String {
    format!("{help}\n{warning}")
}

pub fn page(cx: &mut Cx, tab: usize) {
    use block_i18n::tr;
    let aurora = tr!("ui-options-block-about-land-block");
    let classic = tr!("ui-options-block-about-land-classic");
    match tab {
        0 => cx.group(tr!("ui-options-block-redrawn-windows"), |cx| {
            cx.wide().choice(
                tr!("ui-options-block-conversations"),
                tr!("ui-options-block-conversations-help"),
                &field!(chat_style),
                &[
                    (ChatStyle::Block, tr!("ui-options-block-workspace"), true),
                    (ChatStyle::Compact, tr!("ui-options-block-compact"), true),
                    (ChatStyle::Bubbles, tr!("ui-options-block-bubbles"), true),
                    (ChatStyle::Legacy, tr!("ui-options-block-lines"), false),
                ],
            );
            look(
                cx,
                tr!("ui-options-block-chat-lines"),
                tr!("ui-options-block-chat-lines-help"),
                tr!("ui-options-block-chat-lines-cards"),
                tr!("ui-options-block-chat-lines-classic"),
                &field!(classic_chat_lines),
            );
            cx.wide().choice(
                tr!("ui-options-block-world-map"),
                tr!("ui-options-block-world-map-help"),
                &field!(world_map_style),
                &[
                    (WorldMapStyle::Block, tr!("ui-options-block-world-map-full"), true),
                    (WorldMapStyle::Legacy, tr!("ui-options-block-world-map-side"), false),
                ],
            );
            look(
                cx,
                tr!("ui-options-block-places"),
                tr!("ui-options-block-places-help"),
                tr!("ui-options-block-places-block"),
                tr!("ui-options-block-places-classic"),
                &field!(classic_places),
            );
            cx.wide().choice(
                tr!("ui-options-block-destinations"),
                tr!("ui-options-block-destinations-help"),
                &field!(destinations_style),
                &[
                    (DestinationsStyle::Block, tr!("ui-options-block-destinations-block"), true),
                    (DestinationsStyle::Legacy, tr!("ui-options-block-destinations-classic"), false),
                ],
            );
            look(
                cx,
                tr!("ui-options-block-people"),
                tr!("ui-options-block-people-help"),
                tr!("ui-options-block-people-rows"),
                tr!("ui-options-block-people-table"),
                &field!(classic_people),
            );
            look(
                cx,
                tr!("ui-options-block-profiles"),
                tr!("ui-options-block-profiles-help"),
                tr!("ui-options-block-profiles-block"),
                tr!("ui-options-block-profiles-classic"),
                &field!(classic_profiles),
            );
            let classic_profiles = cx.s.classic_profiles;
            cx.subs(classic_profiles, |cx| {
                cx.toggle(
                    tr!("ui-options-block-profiles-rights"),
                    tr!("ui-options-block-profiles-rights-help"),
                    &field!(profile_rights),
                );
            });
            cx.wide().choice(
                tr!("ui-options-block-photo-window"),
                tr!("ui-options-block-photo-window-help"),
                &field!(photo_window),
                &[
                    (PhotoWindowStyle::Block, tr!("ui-options-block-photo-viewfinder"), true),
                    (PhotoWindowStyle::Panel, tr!("ui-options-block-photo-panel"), true),
                    (PhotoWindowStyle::Legacy, tr!("ui-options-block-photo-classic"), false),
                ],
            );
            let viewfinder = cx.s.photo_window == PhotoWindowStyle::Block;
            cx.subs(!viewfinder, |cx| {
                use crate::snapshot::viewfinder::OnScreen;
                let labels = [
                    (tr!("ui-options-block-vf-controls-always"), true),
                    (tr!("ui-options-block-vf-controls-hover"), true),
                    (tr!("ui-options-block-vf-controls-hidden"), false),
                ];
                let choices: Vec<(OnScreen, &str, bool)> = OnScreen::ALL.iter().zip(labels).map(|(o, (l, a))| (*o, l, a)).collect();
                cx.wide().choice(
                    tr!("ui-options-block-vf-controls"),
                    tr!("ui-options-block-vf-controls-help"),
                    &field!(snapshot.viewfinder.controls),
                    &choices,
                );
            });
            look(
                cx,
                tr!("ui-options-block-about-land"),
                tr!("ui-options-block-about-land-help"),
                aurora,
                classic,
                &field!(classic_about_land),
            );
            look(
                cx,
                tr!("search-title"),
                tr!("ui-options-block-search-help"),
                aurora,
                tr!("ui-options-block-search-classic"),
                &field!(classic_search),
            );
            look(
                cx,
                tr!("region-title"),
                tr!("ui-options-block-region-estate-help"),
                aurora,
                classic,
                &field!(classic_region_estate),
            );
            look(
                cx,
                tr!("common-inventory"),
                tr!("ui-options-block-inventory-help"),
                aurora,
                classic,
                &field!(classic_inventory),
            );
            look(
                cx,
                tr!("ui-options-block-outfit-gallery"),
                tr!("ui-options-block-outfit-gallery-help"),
                tr!("ui-options-block-outfit-gallery-block"),
                tr!("ui-options-block-outfit-gallery-classic"),
                &field!(classic_outfit_gallery),
            );
            look(
                cx,
                tr!("ui-options-block-build"),
                tr!("ui-options-block-build-help"),
                aurora,
                classic,
                &field!(build.legacy_window),
            );
            look(
                cx,
                tr!("ui-options-block-pay-window"),
                tr!("ui-options-block-pay-window-help"),
                tr!("ui-options-block-pay-block"),
                tr!("ui-options-block-pay-classic"),
                &field!(classic_pay_window),
            );
            look(
                cx,
                tr!("money-tracker-title"),
                tr!("ui-options-block-money-tracker-help"),
                aurora,
                classic,
                &field!(legacy_money_tracker),
            );
            look(
                cx,
                tr!("ui-options-block-controls"),
                tr!("ui-options-block-controls-help"),
                tr!("ui-options-block-controls-block"),
                tr!("ui-options-block-controls-classic"),
                &field!(classic_controls),
            );
        }),
        1 => {
            cx.group(tr!("ui-options-block-in-the-world"), |cx| {
                look(
                    cx,
                    tr!("ui-options-block-name-tags"),
                    tr!("ui-options-block-name-tags-help"),
                    tr!("ui-options-block-plates"),
                    tr!("ui-options-block-classic"),
                    &field!(classic_name_tags),
                );
                cx.wide().choice(
                    tr!("ui-options-block-look-at"),
                    tr!("ui-options-block-look-at-help"),
                    &field!(lookat.style),
                    &[
                        (Style::Block, tr!("ui-options-block-halo"), true),
                        (Style::Legacy, tr!("ui-options-block-crosshair"), false),
                    ],
                );
                cx.wide().choice(
                    tr!("ui-options-block-beams"),
                    tr!("ui-options-block-beams-help"),
                    &field!(beams.style),
                    &[
                        (BeamStyle::Block, tr!("ui-options-block-beams-thread"), true),
                        (BeamStyle::Legacy, tr!("ui-options-block-beams-particles"), false),
                    ],
                );
                look(
                    cx,
                    tr!("ui-options-block-rez-effect"),
                    tr!("ui-options-block-rez-effect-help"),
                    tr!("ui-options-block-rez-effect-block"),
                    tr!("ui-options-block-rez-effect-classic"),
                    &field!(classic_rez_effect),
                );
                look(
                    cx,
                    tr!("ui-options-block-hover-tips"),
                    tr!("ui-options-block-hover-tips-help"),
                    aurora,
                    tr!("ui-options-block-hover-tips-legacy"),
                    &field!(classic_hover_tips),
                );
                // also in Graphics › Sky and environment, the same setting
                cx.wide().experimental().choice(
                    tr!("ui-options-colored-sky-ambient"),
                    tr!("ui-options-colored-sky-ambient-help"),
                    &field!(colored_sky_ambient),
                    &[(true, aurora, true), (false, tr!("ui-options-block-classic"), false)],
                );
                // the Legacy Viewer's flat layer until tried on modest GPUs;
                // the two buttons leave it alone
                let help = warned(tr!("ui-options-block-clouds-help"), tr!("ui-options-block-clouds-experimental"));
                cx.wide().experimental().choice(
                    tr!("ui-options-block-clouds"),
                    &help,
                    &field!(volumetric_clouds),
                    &[
                        (true, tr!("ui-options-block-clouds-volumetric"), true),
                        (false, tr!("ui-options-block-clouds-flat"), false),
                    ],
                );
            });
            cx.group(tr!("common-interface"), |cx| {
                cx.wide().choice(
                    tr!("ui-options-block-notifications"),
                    tr!("ui-options-block-notifications-help"),
                    &field!(notifications_style),
                    &[
                        (NotificationsStyle::Block, tr!("ui-options-block-animated-bell"), true),
                        (NotificationsStyle::Legacy, tr!("ui-options-block-toasts"), false),
                    ],
                );
                look(
                    cx,
                    tr!("ui-options-block-bars"),
                    tr!("ui-options-block-bars-help"),
                    tr!("ui-options-block-bars-islands"),
                    tr!("ui-options-block-bars-classic"),
                    &field!(classic_bars),
                );
                look(
                    cx,
                    tr!("ui-options-block-restart-alert"),
                    tr!("ui-options-block-restart-alert-help"),
                    tr!("ui-options-block-restart-card"),
                    tr!("ui-options-block-restart-classic"),
                    &field!(classic_restart_alert),
                );
            });
        }
        _ => {
            cx.group(tr!("ui-options-block-building"), |cx| {
                cx.wide().choice(
                    tr!("ui-options-block-move-hologram"),
                    tr!("ui-options-block-move-hologram-help"),
                    &field!(build.move_hologram),
                    &[
                        (MoveHologram::Off, tr!("ui-options-block-move-hologram-off"), false),
                        (MoveHologram::Moving, tr!("ui-options-block-move-hologram-moving"), true),
                        (MoveHologram::Start, tr!("ui-options-block-move-hologram-start"), true),
                    ],
                );
            });
            cx.group(tr!("ui-options-block-bridge"), |cx| {
                cx.toggle(
                    tr!("ui-options-block-use-bridge"),
                    tr!("ui-options-block-use-bridge-help"),
                    &field!(use_bridge),
                );
            });
            cx.group(tr!("ui-options-block-gaze"), |cx| {
                cx.wide().choice(
                    tr!("ui-options-block-aim"),
                    tr!("ui-options-block-aim-help"),
                    &field!(lookat.aim),
                    &[
                        (Aim::Point, tr!("ui-options-block-aim-point"), true),
                        (Aim::Direction, tr!("ui-options-block-aim-direction"), false),
                    ],
                );
            });
            // the Legacy Viewer's until the human has tried it on the grid,
            // marked experimental meanwhile
            cx.group(tr!("ui-options-block-movement"), |cx| {
                let help = warned(
                    tr!("ui-options-block-own-motion-help"),
                    tr!("ui-options-block-own-motion-experimental"),
                );
                cx.wide().experimental().choice(
                    tr!("ui-options-block-own-motion"),
                    &help,
                    &field!(predict_own_motion),
                    &[
                        (true, tr!("ui-options-block-own-motion-instant"), true),
                        (false, tr!("ui-options-block-own-motion-server"), false),
                    ],
                );
            });
            cx.group(tr!("ui-options-block-animations"), |cx| {
                let help = warned(
                    tr!("ui-options-block-animation-clock-help"),
                    tr!("ui-options-block-animation-clock-experimental"),
                );
                cx.wide().experimental().choice(
                    tr!("ui-options-block-animation-clock"),
                    &help,
                    &field!(synchronize_animations),
                    &[
                        (true, tr!("ui-options-block-animation-clock-synchronized"), true),
                        (false, tr!("ui-options-block-animation-clock-legacy"), false),
                    ],
                );
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn received_filing_stays_opt_in_when_the_looks_change() {
        let mut settings = Settings::default();
        set_all(&mut settings, Look::Block);
        assert!(!settings.sort_received_items);
        settings.sort_received_items = true;
        set_all(&mut settings, Look::Legacy);
        assert!(settings.sort_received_items);
    }

    #[test]
    fn the_two_buttons_set_the_colored_sky_ambient_too() {
        let mut s = Settings::default();
        assert!(!s.colored_sky_ambient, "off by default");
        set_all(&mut s, Look::Block);
        assert!(s.colored_sky_ambient);
        set_all(&mut s, Look::Legacy);
        assert!(!s.colored_sky_ambient);
    }

    #[test]
    fn the_two_buttons_leave_the_volumetric_clouds_as_set() {
        let mut s = Settings::default();
        assert!(!s.volumetric_clouds, "the Legacy Viewer's layer by default");
        set_all(&mut s, Look::Block);
        assert!(!s.volumetric_clouds, "never switched on by the Block button");
        s.volumetric_clouds = true;
        set_all(&mut s, Look::Legacy);
        assert!(s.volumetric_clouds, "nor off by the Legacy one");
        set_all(&mut s, Look::Block);
        assert!(s.volumetric_clouds);
    }

    #[test]
    fn the_block_button_keeps_the_gaze_choice() {
        let mut s = Settings::default();
        set_all(&mut s, Look::Block);
        assert_eq!(s.lookat.aim, Aim::Direction, "never turned to the point by the button");
        s.lookat.aim = Aim::Point;
        set_all(&mut s, Look::Block);
        assert_eq!(s.lookat.aim, Aim::Point, "a chosen point stays");
        set_all(&mut s, Look::Legacy);
        assert_eq!(s.lookat.aim, Aim::Direction);
    }

    #[test]
    fn the_two_buttons_set_every_look_and_leave_the_plates_alone() {
        let mut s = Settings {
            plates_friends_only_full: true,
            ..Settings::default()
        };
        set_all(&mut s, Look::Legacy);
        assert_eq!(s.chat_style, ChatStyle::Legacy);
        assert!(s.classic_name_tags);
        assert_eq!(s.lookat.style, Style::Legacy);
        assert_eq!(s.lookat.aim, Aim::Direction);
        assert_eq!(s.notifications_style, NotificationsStyle::Legacy);
        assert_eq!(s.world_map_style, WorldMapStyle::Legacy);
        assert!(s.classic_bars, "the full-width bars come back");
        assert!(s.classic_people, "People › Near you shows the radar table");
        assert!(s.classic_profiles);
        assert_eq!(s.photo_window, PhotoWindowStyle::Legacy);
        assert!(s.classic_about_land, "About Land opens the Legacy Viewer's nine tabs");
        assert!(s.classic_search, "Search opens the Legacy Viewer's tabs");
        assert!(s.classic_region_estate);
        assert!(s.classic_inventory, "the Legacy Viewer's single inventory tree");
        assert!(s.classic_outfit_gallery, "the Legacy Viewer's small outfit tiles");
        assert!(s.build.legacy_window, "the Legacy Viewer's build tools and tabs");
        assert!(s.classic_places, "Places opens the Legacy Viewer's three tabs");
        assert_eq!(s.destinations_style, DestinationsStyle::Legacy, "the guide's web page");
        assert!(s.classic_pay_window);
        assert!(s.legacy_money_tracker, "the Legacy Viewer's Time / Name / Amount list");
        assert!(s.classic_controls, "the Legacy Viewer's two control windows");
        assert!(s.classic_restart_alert, "the Legacy Viewer's yellow restart box");
        assert!(s.classic_chat_lines, "the Legacy Viewer's one-line chat lines");
        assert!(s.classic_rez_effect, "grey objects while they load");
        assert!(s.classic_hover_tips, "the Legacy Viewer's text tooltips");
        assert!(!s.predict_own_motion, "the avatar waits for the simulator");
        assert!(!s.synchronize_animations);
        assert!(!s.water_effects.enabled);
        assert!(s.plates_friends_only_full, "a plate setting is not a look");
        set_all(&mut s, Look::Block);
        let fresh = Settings::default();
        assert_eq!(s.chat_style, fresh.chat_style);
        assert_eq!(s.classic_name_tags, fresh.classic_name_tags);
        assert_eq!(s.lookat.style, fresh.lookat.style);
        assert_eq!(s.lookat.aim, fresh.lookat.aim);
        assert_eq!(s.notifications_style, NotificationsStyle::Block);
        assert_eq!(fresh.notifications_style, NotificationsStyle::Block);
        assert_eq!(s.world_map_style, fresh.world_map_style);
        assert!(!s.classic_bars && !fresh.classic_bars, "the islands are the default");
        assert!(
            !s.classic_people && !fresh.classic_people,
            "the rows grouped by range are the default"
        );
        assert_eq!(s.classic_profiles, fresh.classic_profiles);
        assert_eq!(s.photo_window, PhotoWindowStyle::Block);
        assert_eq!(fresh.photo_window, PhotoWindowStyle::Block);
        assert_eq!(s.classic_about_land, fresh.classic_about_land);
        assert!(!fresh.classic_about_land, "the Block About Land is the default");
        assert!(!s.classic_search && !fresh.classic_search, "the Block Search is the default");
        assert!(!s.classic_region_estate && !fresh.classic_region_estate);
        assert!(
            !s.classic_inventory && !fresh.classic_inventory,
            "the Block inventory is the default"
        );
        assert!(!s.classic_places && !fresh.classic_places, "Block's Places is the default");
        assert_eq!(s.destinations_style, DestinationsStyle::Block);
        assert_eq!(
            fresh.destinations_style,
            DestinationsStyle::Block,
            "Block's Destinations is the default"
        );
        assert!(
            !s.classic_outfit_gallery && !fresh.classic_outfit_gallery,
            "the showcase is the default"
        );
        assert!(!s.build.legacy_window && !fresh.build.legacy_window);
        assert!(!s.classic_pay_window && !fresh.classic_pay_window);
        assert!(!s.legacy_money_tracker && !fresh.legacy_money_tracker);
        assert!(!s.classic_controls && !fresh.classic_controls);
        assert!(!s.classic_restart_alert && !fresh.classic_restart_alert);
        assert!(!s.classic_chat_lines && !fresh.classic_chat_lines, "the cards are the default");
        assert!(!s.classic_rez_effect && !fresh.classic_rez_effect, "the rez effect is the default");
        assert!(!s.classic_hover_tips && !fresh.classic_hover_tips, "Block's tips are the default");
        assert!(!s.predict_own_motion, "the experimental movement is never a bulk choice");
        assert!(!fresh.predict_own_motion, "the Legacy movement until tried on the grid");
        s.predict_own_motion = true;
        set_all(&mut s, Look::Block);
        assert!(s.predict_own_motion, "a movement chosen on its own row stays");
        assert!(s.synchronize_animations, "the Block bulk choice enables receipt clocks");
        assert!(s.water_effects.enabled, "the explicit Block bulk choice enables water effects");
        assert!(!fresh.water_effects.enabled, "enhanced water is disabled by default");
        assert!(!fresh.synchronize_animations, "Legacy animation timing is the default");
        assert!(s.plates_friends_only_full);
    }

    /// The notifications' style is saved; a file from before it was added
    /// gets the animated bell.
    #[test]
    fn the_notifications_style_is_saved() {
        let s = Settings {
            notifications_style: NotificationsStyle::Legacy,
            ..Settings::default()
        };
        let json = serde_json::to_string(&s).expect("settings serialize");
        let back: Settings = serde_json::from_str(&json).expect("settings read back");
        assert_eq!(back.notifications_style, NotificationsStyle::Legacy);
        let old: Settings = serde_json::from_str("{}").expect("empty settings");
        assert_eq!(old.notifications_style, NotificationsStyle::Block);
    }

    #[test]
    fn the_brand_gradient_uses_block_palette_stops() {
        let p = crate::theme::Theme::default().palette();
        let stops = p.block_gradient();
        assert_eq!(block_color(&p, 0.0), stops[0]);
        assert_eq!(block_color(&p, 1.0), stops[2]);
        assert_eq!(block_color(&p, 0.5), stops[1]);
    }
}
