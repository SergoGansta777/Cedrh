use crate::Position;

use crossterm::{
    cursor,
    event::{read, Event},
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Write};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    pub size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self, std::io::Error> {
        let size = terminal::size().unwrap();
        terminal::enable_raw_mode().ok();

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .ok();
    }

    pub fn quit() {
        Terminal::reset_colors();
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .ok();
        crossterm::terminal::disable_raw_mode().ok();
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;

        stdout().queue(cursor::MoveTo(x - 1, y - 1)).ok();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read() -> Result<Event, std::io::Error> {
        read()
    }

    pub fn cursor_hide() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn cursor_show() {
        stdout().execute(cursor::EnableBlinking).ok();
    }

    pub fn change_cursor() {
        stdout().execute(cursor::SetCursorStyle::BlinkingBar).ok();
    }

    pub fn clear_current_line() {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .ok();
    }

    pub fn reset_colors() {
        stdout().execute(ResetColor).ok();
    }

    pub fn set_bg_color(color: Color) {
        stdout().execute(SetBackgroundColor(color)).ok();
    }

    pub fn reset_bg_color() {
        stdout().execute(SetBackgroundColor(Color::Reset)).ok();
    }

    pub fn set_fg_color(color: Color) {
        stdout().execute(SetForegroundColor(color)).ok();
    }

    pub fn reset_fg_color() {
        stdout().execute(SetForegroundColor(Color::Reset)).ok();
    }
}
