pub mod banner;
mod cli;
mod completer;
mod conversation_selector;
mod display_constants;
mod editor;
mod error;
mod highlighter;
mod info;
mod input;
mod logs;
mod model;
mod oauth_callback;
mod pod;
mod porcelain;
mod prompt;
mod sandbox;
mod state;
mod stream_renderer;
mod sync_display;
pub mod theme;
mod title_display;
mod tools_display;
pub mod tracker;
mod ui;
mod utils;
mod vscode;
mod zsh;

mod update;

use std::sync::LazyLock;

pub use cli::{Cli, ListCommand, ListCommandGroup, PodCommand, PodCommandGroup, TopLevelCommand};
pub use pod::prepare_connect;
pub use pod::provision_for_goal;
pub use sandbox::Sandbox;
pub use title_display::*;
pub use ui::UI;

pub static TRACKER: LazyLock<aimee_tracker::Tracker> =
    LazyLock::new(aimee_tracker::Tracker::default);
