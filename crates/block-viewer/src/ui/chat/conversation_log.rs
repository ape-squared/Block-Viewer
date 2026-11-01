//! Conversation log floater: list past transcripts in-viewer (Legacy
//! LLFloaterConversationLog), then open the History window for one file.
//! Does not dump the user into the OS file explorer.

use crate::theme::Palette;
use crate::world::chat_log::names::{self, EXTENSION, NEARBY};
use egui::RichText;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// One transcript file shown in the list.
#[derive(Clone)]
pub struct Entry {
    pub title: String,
    pub path: PathBuf,
    pub modified: SystemTime,
}

#[derive(Default)]
pub struct ConversationLogUi {
    pub open: bool,
    filter: String,
    entries: Vec<Entry>,
    /// Folder last scanned; refresh when it changes or the window opens.
    scanned: Option<PathBuf>,
    focus_filter: bool,
}

pub enum ConversationLogAction {
    OpenHistory { title: String, path: PathBuf },
    OpenFolder(PathBuf),
}

impl ConversationLogUi {
    pub fn open(&mut self, dir: Option<&Path>) {
        self.open = true;
        self.focus_filter = true;
        self.refresh(dir);
    }

    pub fn refresh(&mut self, dir: Option<&Path>) {
        let Some(dir) = dir else {
            self.entries.clear();
            self.scanned = None;
            return;
        };
        self.scanned = Some(dir.to_path_buf());
        self.entries = list_transcripts(dir);
    }

    pub fn show(&mut self, ctx: &egui::Context, p: &Palette, dir: Option<&Path>) -> Option<ConversationLogAction> {
        if !self.open {
            return None;
        }
        if self.scanned.as_deref() != dir {
            self.refresh(dir);
        }
        let mut action = None;
        let mut open = true;
        let work = super::super::layout::work_area(ctx);
        let size = egui::vec2(420.0, 480.0);
        super::super::widgets::Floater::new(
            "conversation_log",
            block_i18n::tr!("toolbar-command-conversation-log"),
            work.center() - size * 0.5,
            size,
        )
        .show(ctx, p, &mut open, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                let field = ui.add(
                    egui::TextEdit::singleline(&mut self.filter)
                        .id(egui::Id::unique("conversation_log_filter"))
                        .hint_text(block_i18n::tr!("ui-chat-search"))
                        .desired_width((ui.available_width() - 36.0).max(80.0)),
                );
                if std::mem::take(&mut self.focus_filter) {
                    field.request_focus();
                }
                if super::super::widgets::icon_button(ui, p, "refresh", block_i18n::tr!("ui-conversation-log-refresh"), true) {
                    self.refresh(dir);
                }
            });
            ui.add_space(6.0);
            let filter = self.filter.trim().to_lowercase();
            let rows: Vec<&Entry> = self
                .entries
                .iter()
                .filter(|e| filter.is_empty() || e.title.to_lowercase().contains(&filter))
                .collect();
            let h = (ui.available_height() - 36.0).max(80.0);
            egui::Frame::new()
                .fill(p.field)
                .corner_radius(ui.visuals().window_corner_radius)
                .show(ui, |ui| {
                    ui.set_min_height(h);
                    ui.set_width(ui.available_width());
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        if rows.is_empty() {
                            ui.add_space(h * 0.35);
                            ui.vertical_centered(|ui| {
                                let text = if dir.is_none() {
                                    block_i18n::tr!("ui-options-chat-logs-no-account")
                                } else if self.entries.is_empty() {
                                    block_i18n::tr!("ui-conversation-log-empty")
                                } else {
                                    block_i18n::tr!("ui-chat-find-none")
                                };
                                ui.label(RichText::new(text).size(12.5).color(p.muted));
                            });
                            return;
                        }
                        for entry in rows {
                            let resp = ui
                                .add_sized(
                                    egui::vec2(ui.available_width(), 28.0),
                                    egui::Button::new(RichText::new(&entry.title).size(13.0).color(p.ink))
                                        .fill(egui::Color32::TRANSPARENT)
                                        .frame(false),
                                )
                                .on_hover_text(entry.path.display().to_string());
                            if resp.clicked() || resp.double_clicked() {
                                action = Some(ConversationLogAction::OpenHistory {
                                    title: entry.title.clone(),
                                    path: entry.path.clone(),
                                });
                            }
                        }
                    });
                });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(dir) = dir
                        && super::super::widgets::icon_button(
                            ui,
                            p,
                            "folder-open-filled",
                            block_i18n::tr!("ui-options-chat-logs-open"),
                            true,
                        )
                    {
                        action = Some(ConversationLogAction::OpenFolder(dir.to_path_buf()));
                    }
                });
            });
        });
        self.open = open;
        action
    }
}

/// Transcript files in `dir`, newest first; one row per file (dated names
/// keep their day/month so history opens that file).
fn list_transcripts(dir: &Path) -> Vec<Entry> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut entries: Vec<Entry> = read
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if !path.is_file() {
                return None;
            }
            let name = e.file_name().into_string().ok()?;
            let stem = name.strip_suffix(&format!(".{EXTENSION}"))?;
            if stem.is_empty() {
                return None;
            }
            let title = display_title(stem);
            let modified = e.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(SystemTime::UNIX_EPOCH);
            Some(Entry { title, path, modified })
        })
        .collect();
    entries.sort_by(|a, b| b.modified.cmp(&a.modified).then_with(|| a.title.cmp(&b.title)));
    entries
}

fn display_title(stem: &str) -> String {
    if stem == NEARBY || stem.starts_with(&format!("{NEARBY}-")) {
        return block_i18n::tr!("common-nearby-chat").to_owned();
    }
    // Strip dated suffixes: "-2026-10" or "-2026-10-12".
    let base = strip_date_suffix(stem);
    if let Some(name) = base.strip_suffix(names::GROUP_SUFFIX) {
        name.to_owned()
    } else {
        base.to_owned()
    }
}

fn strip_date_suffix(stem: &str) -> &str {
    let bytes = stem.as_bytes();
    // "-YYYY-MM-DD"
    if bytes.len() > 11 {
        let s = &stem[stem.len() - 11..];
        if s.as_bytes()[0] == b'-'
            && s.bytes().enumerate().all(|(i, b)| match i {
                0 | 5 | 8 => b == b'-',
                _ => b.is_ascii_digit(),
            })
        {
            return &stem[..stem.len() - 11];
        }
    }
    // "-YYYY-MM"
    if bytes.len() > 8 {
        let s = &stem[stem.len() - 8..];
        if s.as_bytes()[0] == b'-'
            && s.bytes().enumerate().all(|(i, b)| match i {
                0 | 5 => b == b'-',
                _ => b.is_ascii_digit(),
            })
        {
            return &stem[..stem.len() - 8];
        }
    }
    stem
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_suffixes_are_stripped_for_the_title() {
        assert_eq!(strip_date_suffix("Jane Doe-2026-10"), "Jane Doe");
        assert_eq!(strip_date_suffix("chat-2026-10-12"), "chat");
        assert_eq!(strip_date_suffix("Team (group)-2026-11"), "Team (group)");
        assert_eq!(strip_date_suffix("plain"), "plain");
    }
}
