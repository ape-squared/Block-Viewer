//! Streamer / creator capture mode (immersive chrome, optional fullscreen).

use super::App;

impl App {
    pub(crate) fn set_streamer_mode(&mut self, on: bool) {
        if self.settings.streamer_mode == on {
            if on {
                self.ensure_streamer_chrome();
            }
            return;
        }
        self.apply_streamer_mode(on);
    }

    pub(crate) fn apply_streamer_mode(&mut self, on: bool) {
        self.settings.streamer_mode = on;
        if on {
            let s = self.settings.streamer.clone();
            if s.fullscreen && !self.settings.window_fullscreen {
                self.streamer_forced_fullscreen = true;
                self.settings.window_fullscreen = true;
                self.apply_fullscreen_setting();
            }
            if s.hide_chat_bar && !self.chat_ui.bar_hidden {
                self.streamer_forced_chat_hide = true;
                self.chat_ui.bar_hidden = true;
            }
            if s.hide_huds && self.settings.show_huds && !self.world.rlva_has_locked_hud() {
                self.streamer_forced_huds_off = true;
                self.settings.show_huds = false;
                self.leave_huds();
            }
            self.ensure_streamer_chrome();
        } else {
            if self.streamer_forced_fullscreen {
                self.streamer_forced_fullscreen = false;
                self.settings.window_fullscreen = false;
                self.apply_fullscreen_setting();
            }
            if self.streamer_forced_chat_hide {
                self.streamer_forced_chat_hide = false;
                self.chat_ui.bar_hidden = false;
            }
            if self.streamer_forced_huds_off {
                self.streamer_forced_huds_off = false;
                self.settings.show_huds = true;
            }
            if self.interface.immersive {
                self.interface.set_hidden(&self.egui_ctx, false);
            }
        }
        self.settings.save();
    }

    pub(crate) fn ensure_streamer_chrome(&mut self) {
        if !self.settings.streamer_mode {
            return;
        }
        let hide_huds = self.settings.streamer.hide_huds;
        let immersive = self.settings.streamer.immersive;
        let immersive_huds = self.settings.immersive_huds;
        if (hide_huds || (immersive && !immersive_huds)) && !self.world.rlva_has_locked_hud() {
            self.leave_huds();
        }
        if immersive && !self.camera.mouselook() && !self.interface.immersive {
            self.interface.set_immersive(&self.egui_ctx);
        }
    }

    pub(crate) fn streamer_on(&self) -> bool {
        self.settings.streamer_mode
    }

    pub(crate) fn streamer_hides_balance(&self) -> bool {
        self.streamer_on() && self.settings.streamer.hide_balance
    }

    pub(crate) fn streamer_hides_hover_tips(&self) -> bool {
        self.streamer_on() && self.settings.streamer.hide_hover_tips
    }

    pub(crate) fn streamer_hides_name_tags(&self) -> bool {
        self.streamer_on() && self.settings.streamer.hide_name_tags
    }

    pub(crate) fn streamer_hides_mouselook_esc(&self) -> bool {
        self.streamer_on() && self.settings.streamer.hide_mouselook_esc
    }

    pub(crate) fn streamer_suppresses_toasts(&self) -> bool {
        self.streamer_on() && self.settings.streamer.suppress_toasts
    }
}
