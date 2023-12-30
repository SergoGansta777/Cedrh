use core::time::Duration;
use std::collections::HashMap;
use std::time::Instant;
use std::{env, io::Error};

use crossterm::event::KeyEventKind;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::Color,
};
use std::cell::RefCell;
use unicode_segmentation::UnicodeSegmentation;

use crate::colortheme::get_colors;
use crate::AppArgs;
use crate::Buffer;
use crate::Row;
use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const EDITOR_NAME: &str = env!("CARGO_PKG_NAME");
const ADDITIONAL_QUIT_TIMES: u8 = 1;
const INDENT_COUNT_SPACES: u8 = 4;

#[derive(PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: RefCell<Terminal>,
    cursor_position: Position,
    offset: Position,
    buffer: Buffer,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
    colors: HashMap<String, Color>,
}

impl Editor {
    pub fn new(args: &AppArgs) -> Self {
        let term = env::var("TERM").unwrap_or_default();

        let mut initial_status =
            String::from("HELP: Ctrl-q = quit | Ctrl-s = save | Ctrl-f = find");

        let buffer = if let Some(file_name) = args.file.as_deref() {
            let buf = Buffer::open(file_name);
            if let Ok(buf) = buf {
                buf
            } else {
                initial_status = format!("ERR: Could not open file: {file_name}");
                Buffer::default()
            }
        } else {
            Buffer::default()
        };

        Self {
            should_quit: false,
            terminal: RefCell::new(Terminal::new().expect("Failed to initialize terminal")),
            buffer,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: ADDITIONAL_QUIT_TIMES,
            highlighted_word: None,
            colors: get_colors(&term, args.default_colors),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_event() {
                die(&error);
            }
        }
    }

    #[allow(clippy::as_conversions)]
    fn refresh_screen(&mut self) -> Result<(), Error> {
        self.terminal.borrow_mut().cursor_hide();
        self.terminal
            .borrow_mut()
            .cursor_position(&Position::default());

        if self.should_quit {
            self.terminal.borrow_mut().quit();
        } else {
            self.buffer.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.borrow_mut().size().height as usize),
                ),
            );
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();

            self.terminal.borrow_mut().cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        self.terminal.borrow_mut().cursor_show();
        self.terminal.borrow_mut().flush()
    }

    #[allow(clippy::as_conversions)]
    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.borrow_mut().size().width as usize;
        let modified_indicator = if self.buffer.is_modificated() {
            " (modified)"
        } else {
            ""
        };
        let mut file_name = "[No Name]".to_owned();
        if let Some(name) = self.buffer.file_name() {
            file_name = (*name).clone();
            file_name.truncate(20);
        }

        status = format!(
            "{} - {} lines{}",
            file_name,
            self.buffer.len(),
            modified_indicator
        );
        let line_indicator = format!(
            "{} | {}/{}",
            self.buffer.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.buffer.len()
        );

        #[allow(clippy::arithmetic_side_effects)]
        let len = status.len() + line_indicator.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{status}{line_indicator}");
        status.truncate(width);
        self.terminal.borrow_mut().set_bg_color(
            *self
                .colors
                .get("active_border_color")
                .unwrap_or(&self.colors["background"]),
        );
        self.terminal
            .borrow_mut()
            .set_fg_color(self.colors["foreground"]);
        println!("{status}\r");
        self.terminal.borrow_mut().reset_fg_color();
        self.terminal.borrow_mut().reset_bg_color();
    }

    #[allow(clippy::as_conversions)]
    fn draw_message_bar(&self) {
        self.terminal.borrow_mut().clear_current_line();
        let message = &self.status_message;
        if message.time.elapsed() < Duration::new(45, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.borrow_mut().size().width as usize);
            print!("{text}");
        }
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, Error>
    where
        C: FnMut(&mut Self, KeyEvent, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{prompt}{result}"));
            self.refresh_screen()?;

            let event = Terminal::read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Backspace => {
                            let graphemes_count = result.graphemes(true).count();
                            result = result
                                .graphemes(true)
                                .take(graphemes_count.saturating_sub(1))
                                .collect();
                        }
                        KeyCode::Enter => break,
                        KeyCode::Char(ch) => {
                            if !ch.is_control() {
                                result.push(ch);
                            }
                        }
                        KeyCode::Esc => {
                            result.truncate(0);
                            break;
                        }
                        _ => (),
                    }
                    callback(self, key, &result);
                }
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }

    fn save(&mut self) {
        if self.buffer.file_name().is_none() {
            let new_name = self.prompt("Save as ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_owned());
                return;
            }
            self.buffer.set_file_name(new_name);
        }

        if self.buffer.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_owned());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_owned());
        }
    }

    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Search (ESC to cancel, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key.code {
                        KeyCode::Right | KeyCode::Down => {
                            direction = SearchDirection::Forward;
                            editor.move_cursor(KeyCode::Right);
                            moved = true;
                        }
                        KeyCode::Left | KeyCode::Up => direction = SearchDirection::Backward,
                        _ => direction = SearchDirection::Forward,
                    }
                    if let Some(position) =
                        editor
                            .buffer
                            .find(query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(KeyCode::Left);
                    }
                    editor.highlighted_word = Some(query.to_owned());
                },
            )
            .unwrap_or(None);
        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
        self.highlighted_word = None;
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn process_event(&mut self) -> Result<(), Error> {
        let event = Terminal::read()?;

        match event {
            Event::Key(pressed_key) if pressed_key.kind == KeyEventKind::Press => {
                self.process_keypress(pressed_key)
            }
            Event::Resize(width, height) => {
                self.terminal.borrow_mut().size.width = width;
                self.terminal.borrow_mut().size.height =
                    height - if env::consts::OS == "windows" { 1 } else { 2 };
                Ok(())
            }
            _ => Ok(()),
        }
    }

    #[allow(clippy::unnecessary_wraps, clippy::arithmetic_side_effects)]
    fn process_keypress(&mut self, pressed_key: KeyEvent) -> Result<(), Error> {
        match (pressed_key.modifiers, pressed_key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                if self.quit_times > 0 && self.buffer.is_modificated() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! file has unsaved changes. Press Ctrl_q {} more times to quit",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true;
            }
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (KeyModifiers::CONTROL, KeyCode::Char('f')) => self.search(),
            (KeyModifiers::NONE, KeyCode::Tab) => {
                for _ in 0..INDENT_COUNT_SPACES {
                    self.buffer.insert(&self.cursor_position, ' ');
                    self.move_cursor(KeyCode::Right);
                }
            }
            (_, KeyCode::Char(ch)) => {
                self.buffer.insert(&self.cursor_position, ch);
                self.move_cursor(KeyCode::Right);
            }
            (KeyModifiers::NONE, KeyCode::Delete) => self.buffer.delete(&self.cursor_position),
            (KeyModifiers::NONE, KeyCode::Backspace) => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(KeyCode::Left);
                    self.buffer.delete(&self.cursor_position);
                }
            }
            (KeyModifiers::NONE, KeyCode::Enter) => {
                self.buffer.insert(&self.cursor_position, '\n');
                self.move_cursor(KeyCode::Right);
            }
            (
                _,
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home,
            ) => self.move_cursor(pressed_key.code),
            _ => (),
        }
        self.scroll();
        if self.quit_times < ADDITIONAL_QUIT_TIMES {
            self.quit_times = ADDITIONAL_QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }

        Ok(())
    }

    #[allow(clippy::as_conversions)]
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.borrow_mut().size().width as usize;
        let height = self.terminal.borrow_mut().size().height as usize;
        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    #[allow(clippy::as_conversions, clippy::arithmetic_side_effects)]
    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.borrow_mut().size().height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.buffer.len();
        let mut width = if let Some(row) = self.buffer.row(y) {
            row.len()
        } else {
            0
        };

        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.buffer.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.buffer.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let info_message = format!("{EDITOR_NAME} editor --version {VERSION}");
        let welcome_messages = [info_message.as_str(), "Welcome to the club!"];
        for message in &welcome_messages {
            self.draw_centered_message(message);
        }
    }

    #[allow(clippy::as_conversions)]
    fn draw_centered_message(&self, message: &str) {
        let width = self.terminal.borrow_mut().size().width as usize;
        let len = message.len();

        #[allow(clippy::arithmetic_side_effects, clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        let welcome_message = format!("~{spaces}{message}");
        let truncated_message = welcome_message.chars().take(width).collect::<String>();

        println!("{truncated_message}\r");
    }

    #[allow(clippy::as_conversions)]
    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.borrow_mut().size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(&self.colors, start, end);
        println!("{row}\r");
    }

    #[allow(
        clippy::integer_division,
        clippy::arithmetic_side_effects,
        clippy::as_conversions
    )]
    fn draw_rows(&self) {
        let height = self.terminal.borrow_mut().size().height;
        for terminal_row in 0..height {
            self.terminal.borrow_mut().clear_current_line();

            match self
                .buffer
                .row(self.offset.y.saturating_add(terminal_row as usize))
            {
                Some(row) => self.draw_row(row),
                None if self.buffer.is_empty() && terminal_row == height / 3 => {
                    self.draw_welcome_message();
                }
                None => println!("~\r"),
            }
        }
    }
}

fn die(error: &Error) {
    panic!("{}", error);
}
