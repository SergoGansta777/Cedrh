#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::question_mark_used,
    clippy::single_call_fn,
    clippy::single_match,
    clippy::panic
)]

pub use args::AppArgs;
pub use buffer::Buffer;
pub use clap::Parser;
pub use editor::Position;
pub use editor::SearchDirection;
pub use filetype::FileType;
pub use filetype::HighlightingOptions;
pub use row::Row;
pub use terminal::Terminal;

use editor::Editor;

mod args;
mod buffer;
mod colortheme;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

fn main() {
    let args = AppArgs::parse();
    Editor::new(&args).run();
}
