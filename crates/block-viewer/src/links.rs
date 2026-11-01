//! Project links shown on the login screen. Empty = not configured yet: the
//! button is shown disabled and nothing is fetched. The README header carries
//! the same Discord and website links: change them there too.

/// Source code repository (GitHub page). Empty until the Block Viewer remote exists.
pub const GITHUB: &str = "https://github.com/ape-squared/Block";
/// Issue template chooser, opened in the browser by "Report a bug" (login
/// screen, top bar, Help menu) and by the crash window.
pub const NEW_ISSUE: &str = "https://github.com/ape-squared/Block/issues/new/choose";
/// The project's wiki (optional).
pub const WIKI: &str = "https://github.com/ape-squared/Block/wiki";
/// Community Discord invite.
pub const DISCORD: &str = "https://discord.gg/s96h8KwES7";
/// Project website (Block product site until a viewer microsite exists).
pub const WEBSITE: &str = "https://ape-squared.github.io/Block";
/// The privacy policy on the website (Preferences › Privacy › Your data).
pub const PRIVACY_POLICY: &str = "https://ape-squared.github.io/Block/privacy/";
/// News feed: this folder of `main`, hand-edited Markdown files, one
/// `<language>.md` per UI language (`news/` at the repository root).
pub const NEWS_FEED: &str = "";
/// Files of the source repository as GitHub serves them unrendered; a branch
/// name and a path follow.
pub const RAW_FILES: &str = "https://raw.githubusercontent.com/ape-squared/Block/";
