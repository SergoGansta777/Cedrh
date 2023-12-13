use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Error, Write};
use std::usize;

use crate::FileType;
use crate::Position;
use crate::Row;
use crate::SearchDirection;

#[derive(Default)]
pub struct Buffer {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    modificated: bool,
    file_type: FileType,
}

impl Buffer {
    pub fn open(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let file_type = FileType::from(filename);
        let rows = reader
            .lines()
            .map(|line_result| line_result.map(|line| Row::from(line.as_str())))
            .collect::<Result<Vec<_>, io::Error>>()?;

        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            modificated: false,
            file_type,
        })
    }

    #[must_use]
    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        #[allow(clippy::arithmetic_side_effects)]
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }
        self.modificated = true;
        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
        }
        self.unhighlight_rows(at.y);
    }

    fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for row in self.rows.iter_mut().skip(start) {
            row.is_highlighted = false;
        }
    }

    #[allow(clippy::arithmetic_side_effects, clippy::indexing_slicing)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y >= len {
            return;
        }
        self.modificated = true;
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
        self.unhighlight_rows(at.y);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let file = File::create(file_name)?;
            let mut writer = BufWriter::new(file);

            for row in self.rows.iter() {
                writer.write_all(row.as_bytes())?;
                writer.write_all(b"\n")?;
            }

            self.modificated = false;
            writer.flush()?;
        }
        Ok(())
    }

    #[must_use]
    pub fn is_modificated(&self) -> bool {
        self.modificated
    }

    //TODO: refactor this
    #[allow(clippy::indexing_slicing)]
    #[must_use]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        let mut position = Position { x: at.x, y: at.y };

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };
        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;
        let until = if let Some(until) = until {
            if until.saturating_add(1) < self.rows.len() {
                until.saturating_add(1)
            } else {
                self.rows.len()
            }
        } else {
            self.rows.len()
        };
        #[allow(clippy::indexing_slicing)]
        for row in &mut self.rows[..until] {
            start_with_comment = row.highlight(
                self.file_type.highlighting_options(),
                word,
                start_with_comment,
            );
        }
    }
}
