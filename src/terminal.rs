use crate::Position;

use crossterm::{
    cursor,
    event::{read, Event},
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    QueueableCommand,
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
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))
            .ok();
    }

    pub fn quit(&mut self) {
        self.reset_colors();
        self.clear_screen();
        println!("Goodbye!\r");
        terminal::disable_raw_mode().ok();
    }

    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    pub fn cursor_position(&mut self, position: &Position) {
        let Position { mut x, mut y } = *position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;

        self.stdout
            .queue(cursor::MoveTo(x.saturating_sub(1), y.saturating_sub(1)))
            .ok();
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.stdout.flush()
    }

    pub fn read() -> Result<Event, Error> {
        read()
    }

    pub fn cursor_hide(&mut self) {
        self.stdout.queue(cursor::Hide).ok();
    }

    pub fn cursor_show(&mut self) {
        self.stdout.queue(cursor::Show).ok();
    }

    pub fn enable_alternative_screen(&mut self) {
        self.stdout.queue(EnterAlternateScreen).ok();
    }

    pub fn disable_alternative_screen(&mut self) {
        self.stdout.queue(LeaveAlternateScreen).ok();
    }

    pub fn write_row(&mut self, row: &str) {
        self.stdout.queue(crossterm::style::Print(row)).ok();
        self.stdout.queue(crossterm::style::Print("\r")).ok();
    }

    pub fn add_new_line(&mut self) {
        self.stdout.queue(crossterm::style::Print("\n")).ok();
    }

    pub fn clear_current_line(&mut self) {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .ok();
    }

    pub fn reset_colors(&mut self) {
        self.stdout.queue(ResetColor).ok();
    }

    pub fn set_bg_color(&mut self, color: Color) {
        self.stdout.queue(SetBackgroundColor(color)).ok();
    }

    pub fn reset_bg_color(&mut self) {
        self.stdout.queue(SetBackgroundColor(Color::Reset)).ok();
    }

    pub fn set_fg_color(&mut self, color: Color) {
        self.stdout.queue(SetForegroundColor(color)).ok();
    }

    pub fn reset_fg_color(&mut self) {
        self.stdout.queue(SetForegroundColor(Color::Reset)).ok();
    }
}
