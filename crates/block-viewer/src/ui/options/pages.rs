//! The pages of the preferences, one module each, in the order of the
//! sidebar. A page declares its rows through `Cx` for one of its tabs.

pub mod accessibility;
pub mod appearance;
pub mod avatars;
pub mod block;
pub mod camera;
pub mod chat;
pub mod commands;
pub mod confirmations;
pub mod controls;
pub mod fun;
pub mod graphics;
pub mod inventory;
pub mod joystick;
pub mod layout;
pub mod media;
pub mod network;
pub mod privacy;
pub mod protections;
pub mod rlva;
pub mod shortcuts;
pub mod sound;
pub mod spelling;
pub mod system;

use super::Page;
use super::cx::Cx;

/// The tabs of a page.
pub fn tabs(page: Page) -> Vec<&'static str> {
    match page {
        Page::Aurora => vec![
            block_i18n::tr!("common-windows"),
            block_i18n::tr!("ui-options-tab-world-and-interface"),
            block_i18n::tr!("common-behavior"),
        ],
        Page::Graphics => vec![
            block_i18n::tr!("ui-options-quality"),
            block_i18n::tr!("common-light"),
            block_i18n::tr!("ui-options-tab-sky-and-environment"),
            block_i18n::tr!("ui-options-tab-reflections-and-water"),
            block_i18n::tr!("common-image"),
            block_i18n::tr!("ui-options-smoothness"),
            block_i18n::tr!("region-parcels"),
        ],
        Page::Camera => vec![
            block_i18n::tr!("common-viewpoint"),
            block_i18n::tr!("common-behavior"),
            block_i18n::tr!("common-mouselook"),
        ],
        Page::Controls => vec![block_i18n::tr!("ui-options-page-controls"), block_i18n::tr!("joystick-title")],
        Page::Avatars => vec![
            block_i18n::tr!("ui-options-tab-name-tags"),
            block_i18n::tr!("ui-options-names"),
            block_i18n::tr!("ui-options-look-at-targets"),
            block_i18n::tr!("ui-options-block-animations"),
        ],
        Page::Chat => vec![
            block_i18n::tr!("common-nearby-chat"),
            block_i18n::tr!("ui-options-block-notifications"),
            block_i18n::tr!("ui-options-tab-keywords"),
            block_i18n::tr!("ui-options-tab-history"),
            block_i18n::tr!("ui-options-tab-away-and-replies"),
            block_i18n::tr!("ui-options-groups-and-blocking"),
            block_i18n::tr!("ui-options-tab-spelling"),
        ],
        Page::Sound => vec![
            block_i18n::tr!("ui-options-volumes"),
            block_i18n::tr!("ui-options-interface-sounds"),
            block_i18n::tr!("common-voice"),
        ],
        Page::Layout => vec![
            block_i18n::tr!("ui-options-tab-bars-and-windows"),
            block_i18n::tr!("ui-options-hover-tips"),
            block_i18n::tr!("ui-options-tab-startup"),
        ],
        Page::Appearance => vec![
            block_i18n::tr!("ui-options-font"),
            block_i18n::tr!("ui-options-colors"),
            block_i18n::tr!("extra-skin"),
        ],
        Page::Shortcuts => shortcuts::tabs(),
        _ => vec![page.name()],
    }
}

/// The rows of one tab of a page.
pub fn draw(cx: &mut Cx, page: Page, tab: usize) {
    match page {
        Page::Aurora => block::page(cx, tab),
        Page::Graphics => graphics::page(cx, tab),
        Page::Camera => camera::page(cx, tab),
        Page::Avatars => avatars::page(cx, tab),
        Page::Chat => chat::page(cx, tab),
        Page::Sound => sound::page(cx, tab),
        Page::Media => media::page(cx),
        Page::Privacy => privacy::page(cx),
        Page::Protections => protections::page(cx),
        Page::Rlva => rlva::page(cx),
        Page::Layout => layout::page(cx, tab),
        Page::Appearance => appearance::page(cx, tab),
        Page::Accessibility => accessibility::page(cx),
        Page::Confirmations => confirmations::page(cx),
        Page::Inventory => inventory::page(cx),
        Page::Controls => {
            if tab == 0 {
                controls::page(cx)
            } else {
                joystick::page(cx)
            }
        }
        Page::Shortcuts => shortcuts::page(cx, tab),
        Page::Commands => commands::page(cx),
        Page::System => system::page(cx),
        Page::Network => network::page(cx),
        Page::Fun => fun::page(cx),
    }
}
