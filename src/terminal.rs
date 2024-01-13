use crate::Position;

use crossterm::{
    cursor,
    event::{read, Event},
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Error, Stdout, Write};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[non_exhaustive]
pub struct Terminal {
    pub size: Size,
    pub stdout: Stdout,
}

impl Terminal {
    #[allow(clippy::unnecessary_wraps)]
    pub fn new() -> Result<Self, Error> {
        let size = terminal::size().unwrap_or((40, 40));
        terminal::enable_raw_mode().ok();

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            stdout: io::stdout(),
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen(&mut self) {
        queue!(self.stdout, Clear(terminal::ClearType::All)).ok();
    }

    pub fn quit(&mut self) {
        self.reset_colors();
        self.clear_screen();
        terminal::disable_raw_mode().ok();

        self.flush().ok();
    }

    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    pub fn cursor_position(&mut self, position: &Position) {
        let Position { mut x, mut y } = *position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;

        queue!(
            self.stdout,
            cursor::MoveTo(x.saturating_sub(1), y.saturating_sub(1))
        )
        .ok();

        self.stdout.flush().ok();
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.stdout.flush()
    }

    pub fn read() -> Result<Event, Error> {
        read()
    }

    pub fn cursor_hide(&mut self) {
        queue!(self.stdout, cursor::Hide).ok();
    }

    pub fn cursor_show(&mut self) {
        queue!(self.stdout, cursor::Show).ok();
    }

    pub fn enable_alternative_screen(&mut self) {
        queue!(self.stdout, EnterAlternateScreen).ok();
    }

    pub fn disable_alternative_screen(&mut self) {
        queue!(self.stdout, LeaveAlternateScreen).ok();
    }

    pub fn write_row(&mut self, row: &str, new_line: bool) {
        queue!(
            self.stdout,
            Print(row),
            Clear(terminal::ClearType::UntilNewLine),
            Print("\r"),
        )
        .ok();
        if new_line {
            queue!(self.stdout, Print("\n")).ok();
        }
    }

    pub fn reset_colors(&mut self) {
        queue!(self.stdout, ResetColor).ok();
    }

    pub fn set_bg_color(&mut self, color: Color) {
        queue!(self.stdout, SetBackgroundColor(color)).ok();
    }

    pub fn reset_bg_color(&mut self) {
        queue!(self.stdout, SetBackgroundColor(Color::Reset)).ok();
    }

    pub fn set_fg_color(&mut self, color: Color) {
        queue!(self.stdout, SetForegroundColor(color)).ok();
    }

    pub fn reset_fg_color(&mut self) {
        queue!(self.stdout, SetForegroundColor(Color::Reset)).ok();
    }
}
