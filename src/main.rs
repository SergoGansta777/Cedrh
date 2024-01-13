#![warn(clippy::all, clippy::pedantic)]
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
    clippy::panic,
    clippy::std_instead_of_alloc,
    clippy::expect_used,
    clippy::indexing_slicing
)]

use args::AppArgs;
use buffer::Buffer;
use clap::Parser;
use editor::Position;
use editor::SearchDirection;
use filetype::FileType;
use filetype::HighlightingOptions;
use row::Row;
use terminal::Terminal;

use editor::Editor;

mod args;
mod buffer;
mod colortheme;
mod editor;
mod filetype;
mod highlighting;
mod row;
mod terminal;

#[cfg(target_os = "windows")]
fn enable_virtual_terminal_processing() -> Result<(), Error> {
    use crossterm::terminal::enable_ansi_support;
    enable_ansi_support()
}

fn main() {
    #[cfg(target_os = "windows")]
    enable_virtual_terminal_processing().ok();
    let args = AppArgs::parse();
    Editor::new(&args).run();
}
