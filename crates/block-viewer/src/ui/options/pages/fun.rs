//! Preferences › Just for fun: name-plate toys and optional Lovense Connect.

use super::super::cx::{Cx, Nothing, field};

pub fn page(cx: &mut Cx) {
    use block_i18n::tr;
    let p = cx.p;
    cx.group(tr!("ui-options-fun-plates"), |cx| {
        if cx.s.classic_name_tags {
            cx.note(tr!("ui-options-fun-classic-tags"));
        }
        if cx.s.reduce_motion {
            cx.note(tr!("ui-options-fun-reduced-motion"));
        }
        cx.toggle(
            tr!("ui-options-fun-jiggle"),
            tr!("ui-options-fun-jiggle-help"),
            &field!(plates_jiggle),
        );
        cx.toggle(
            tr!("ui-options-fun-click-combo"),
            tr!("ui-options-fun-click-combo-help"),
            &field!(plates_click_combo),
        );
        cx.wide()
            .row(tr!("ui-options-fun-try"), tr!("ui-options-fun-try-help"), &Nothing, |ui, s| {
                crate::ui::nameplate::playground::show(ui, &p, s.plates_jiggle, s.plates_click_combo, s.reduce_motion);
                false
            });
    });
    cx.group(tr!("ui-options-lovense"), |cx| {
        cx.note(tr!("ui-options-lovense-help"));
        cx.toggle(
            tr!("ui-options-lovense-enable"),
            tr!("ui-options-lovense-enable-help"),
            &field!(lovense.enabled),
        );
        cx.toggle(
            tr!("ui-options-lovense-chat"),
            tr!("ui-options-lovense-chat-help"),
            &field!(lovense.chat_commands),
        );
        let off = !cx.s.lovense.enabled;
        cx.subs(off, |cx| {
            cx.row(
                tr!("ui-options-lovense-endpoint"),
                tr!("ui-options-lovense-endpoint-help"),
                &field!(lovense.endpoint),
                |ui, s| {
                    ui.add(
                        egui::TextEdit::singleline(&mut s.lovense.endpoint)
                            .hint_text("http://127.0.0.1:20010/command")
                            .desired_width(280.0),
                    )
                    .changed()
                },
            );
            cx.row(
                tr!("ui-options-lovense-max"),
                tr!("ui-options-lovense-max-help"),
                &field!(lovense.max_strength),
                |ui, s| {
                    let mut n = s.lovense.max_strength as u32;
                    let ch = ui.add(egui::Slider::new(&mut n, 1..=20).trailing_fill(true)).changed();
                    s.lovense.max_strength = n as u8;
                    ch
                },
            );
        });
    });
}
