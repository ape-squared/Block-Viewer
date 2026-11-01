//! Project links shown on the login screen. Empty = not configured yet: the
//! button is shown disabled and nothing is fetched. Keep these on the
//! Block Viewer repo/site — not the Block-GPPI injector sister project.

/// Source code repository (GitHub).
pub const GITHUB: &str = "https://github.com/ape-squared/Block-Viewer";
/// Issue template chooser, opened in the browser by "Report a bug" (login
/// screen, top bar, Help menu) and by the crash window.
pub const NEW_ISSUE: &str = "https://github.com/ape-squared/Block-Viewer/issues/new/choose";
/// Feature request template (Help / about). Kept public for menus and docs.
#[allow(dead_code)]
pub const FEATURE_REQUEST: &str = "https://github.com/ape-squared/Block-Viewer/issues/new?template=feature_request.yml";
/// The project's wiki (optional).
pub const WIKI: &str = "https://github.com/ape-squared/Block-Viewer/wiki";

/// Community Discord invite.
pub const DISCORD: &str = "https://discord.gg/s96h8KwES7";
/// Project website (GitHub Pages).
pub const WEBSITE: &str = "https://ape-squared.github.io/Block-Viewer/";
/// The privacy policy on the website (Preferences › Privacy › Your data).
pub const PRIVACY_POLICY: &str = "https://ape-squared.github.io/Block-Viewer/privacy/";
/// News feed: this folder of `main`, hand-edited Markdown files, one
/// `<language>.md` per UI language (`news/` at the repository root).
pub const NEWS_FEED: &str = "";
/// Files of the source repository as GitHub serves them unrendered; a branch
/// name and a path follow.
pub const RAW_FILES: &str = "https://raw.githubusercontent.com/ape-squared/Block-Viewer/";
