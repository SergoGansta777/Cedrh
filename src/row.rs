use core::cmp;
use std::borrow::Cow;
use std::collections::HashMap;

use crossterm::{style::Color, style::SetForegroundColor};
use unicode_segmentation::UnicodeSegmentation;

use crate::highlighting;
use crate::HighlightingOptions;
use crate::SearchDirection;

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    is_highlighted: bool,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
            is_highlighted: false,
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    #[must_use]
    #[allow(clippy::arithmetic_side_effects, clippy::string_slice)]
    pub fn render(&self, colors: &HashMap<String, Color>, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::with_capacity(end - start);
        let mut current_highlighting: Cow<highlighting::Type> =
            Cow::Borrowed(&highlighting::Type::None);

        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            let highlighting_type = self
                .highlighting
                .get(index)
                .unwrap_or(&highlighting::Type::None);

            if highlighting_type != &*current_highlighting {
                current_highlighting = Cow::Borrowed(highlighting_type);
                result.push_str(
                    format!("{}", SetForegroundColor(highlighting_type.to_color(colors))).as_str(),
                );
            }

            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }

        result.push_str(format!("{}", SetForegroundColor(Color::Reset)).as_str());
        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_highlighted(&mut self) -> &mut bool {
        &mut self.is_highlighted
    }

    #[allow(clippy::arithmetic_side_effects, clippy::pattern_type_mismatch)]
    pub fn insert(&mut self, at: usize, ch: char) {
        if at >= self.len() {
            self.string.push(ch);
        } else {
            let grapheme_indices: Vec<_> = self.string.grapheme_indices(true).collect();
            if let Some((byte_index, _)) = grapheme_indices.get(at) {
                self.string.insert_str(*byte_index, &ch.to_string());
            } else {
                self.string.push(ch);
            }
        }
        self.len += 1;
    }

    #[allow(clippy::arithmetic_side_effects)]
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    #[allow(clippy::arithmetic_side_effects, clippy::string_slice)]
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut result = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    #[must_use]
    #[allow(clippy::string_slice)]
    pub fn split(&mut self, at: usize) -> Self {
        let byte_index = self
            .string
            .grapheme_indices(true)
            .nth(at)
            .map_or(self.string.len(), |(index, _)| index);

        let splitted_row = self.string.get(byte_index..).unwrap_or("").to_owned();
        self.string.truncate(byte_index);

        let length = self.string.graphemes(true).count();
        let splitted_length = splitted_row.graphemes(true).count();

        self.len = length;
        self.is_highlighted = false;

        Self {
            string: splitted_row,
            highlighting: Vec::new(),
            is_highlighted: false,
            len: splitted_length,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    #[must_use]
    #[allow(clippy::arithmetic_side_effects, clippy::string_slice)]
    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring[..].grapheme_indices(true).enumerate()
            {
                if matching_byte_index == byte_index {
                    #[allow(clippy::arithmetic_side_effects)]
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    #[allow(clippy::string_slice, clippy::pattern_type_mismatch)]
    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }
            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count())
                {
                    #[allow(clippy::indexing_slicing)]
                    for i in search_match..next_index {
                        self.highlighting[i] = highlighting::Type::Match;
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        hl_type: highlighting::Type,
    ) -> bool {
        if substring.is_empty() {
            return false;
        }
        for (substring_index, ch) in substring.chars().enumerate() {
            if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
                if *next_char != ch {
                    return false;
                }
            } else {
                return false;
            }
        }
        for _ in 0..substring.len() {
            self.highlighting.push(hl_type);
            *index += 1;
        }
        true
    }

    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: highlighting::Type,
    ) -> bool {
        if *index > 0 {
            #[allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]
            let prev_char = chars[*index - 1];
            if !is_separator(prev_char) {
                return false;
            }
        }
        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                #[allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]
                let next_char = chars[*index + word.len()];
                if !is_separator(next_char) {
                    continue;
                }
            }
            if self.highlight_str(index, word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.primary_keywords(),
            highlighting::Type::PrimaryKeywords,
        )
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.secondary_keywords(),
            highlighting::Type::SecondaryKeywords,
        )
    }

    fn highlight_operators(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        for word in opts.operators() {
            if self.highlight_str(index, word, chars, highlighting::Type::Operators) {
                return true;
            }
        }
        false
    }
    fn highlight_brackets(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        for word in opts.brackets() {
            if self.highlight_str(index, word, chars, highlighting::Type::Brackets) {
                return true;
            }
        }
        false
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        ch: char,
        chars: &[char],
    ) -> bool {
        if opts.characters() && ch == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };
                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in 0..=closing_index.saturating_sub(*index) {
                            self.highlighting.push(highlighting::Type::Character);
                            *index += 1;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        ch: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && ch == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(highlighting::Type::Comment);
                        *index += 1;
                    }
                    return true;
                }
            };
        }
        false
    }

    #[allow(
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects,
        clippy::string_slice
    )]
    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        ch: char,
        chars: &[char],
    ) -> bool {
        if opts.multiline_comments() && ch == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index =
                        if let Some(closing_index) = self.string[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };
                    for _ in *index..closing_index {
                        self.highlighting.push(highlighting::Type::MultilineComment);
                        *index += 1;
                    }
                    return true;
                }
            }
        }
        false
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        ch: char,
        chars: &[char],
    ) -> bool {
        if opts.strings() && ch == '"' {
            loop {
                self.highlighting.push(highlighting::Type::String);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        if let Some(prev_char) = chars.get(*index - 1) {
                            if *prev_char != '\\' {
                                break;
                            }
                        };
                    }
                } else {
                    break;
                }
            }
            self.highlighting.push(highlighting::Type::String);
            *index += 1;
            return true;
        }
        false
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        ch: char,
        chars: &[char],
    ) -> bool {
        if opts.numbers() && ch.is_ascii_digit() {
            if *index > 0 {
                #[allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]
                let prev_char = chars[*index - 1];
                if !is_separator(prev_char) {
                    return false;
                }
            }
            loop {
                self.highlighting.push(highlighting::Type::Number);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }
        false
    }

    #[allow(
        clippy::indexing_slicing,
        clippy::string_slice,
        clippy::arithmetic_side_effects
    )]
    pub fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.string.chars().collect();
        if self.is_highlighted && word.is_none() {
            return false;
        }
        self.highlighting = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;
        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };
            for _ in 0..closing_index {
                self.highlighting.push(highlighting::Type::MultilineComment);
            }
            index = closing_index;
        }
        while let Some(ch) = chars.get(index) {
            if self.highlight_multiline_comment(&mut index, opts, *ch, &chars) {
                in_ml_comment = true;
                continue;
            }
            in_ml_comment = false;
            if self.highlight_char(&mut index, opts, *ch, &chars)
                || self.highlight_comment(&mut index, opts, *ch, &chars)
                || self.highlight_primary_keywords(&mut index, opts, &chars)
                || self.highlight_secondary_keywords(&mut index, opts, &chars)
                || self.highlight_operators(&mut index, opts, &chars)
                || self.highlight_string(&mut index, opts, *ch, &chars)
                || self.highlight_number(&mut index, opts, *ch, &chars)
                || self.highlight_brackets(&mut index, opts, &chars)
            {
                continue;
            }
            self.highlighting.push(highlighting::Type::None);
            index += 1;
        }
        self.highlight_match(word);
        if in_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }
        self.is_highlighted = true;
        false
    }
}

fn is_separator(ch: char) -> bool {
    (ch.is_ascii_punctuation() && ch != '_') || ch.is_ascii_whitespace()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_highlight_find() {
        let mut row = Row::from("1testtest");
        row.highlighting = vec![
            highlighting::Type::Number,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
            highlighting::Type::None,
        ];
        row.highlight_match(&Some("t".to_owned()));
        assert_eq!(
            vec![
                highlighting::Type::Number,
                highlighting::Type::Match,
                highlighting::Type::None,
                highlighting::Type::None,
                highlighting::Type::Match,
                highlighting::Type::Match,
                highlighting::Type::None,
                highlighting::Type::None,
                highlighting::Type::Match
            ],
            row.highlighting
        );
    }

    #[test]
    fn test_find() {
        let row = Row::from("1testtest");
        assert_eq!(row.find("t", 0, SearchDirection::Forward), Some(1));
        assert_eq!(row.find("t", 2, SearchDirection::Forward), Some(4));
        assert_eq!(row.find("t", 5, SearchDirection::Forward), Some(5));
    }
}
