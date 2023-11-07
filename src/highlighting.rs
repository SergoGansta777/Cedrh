use std::collections::HashMap;

use termion::color;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
    Operators,
    Brackets,
}

impl Type {
    pub fn to_color(self, colors: &HashMap<String, color::Rgb>) -> impl color::Color {
        match self {
            Type::Number => colors["color1"],
            Type::Match => colors["color0"],
            Type::String => colors["color2"],
            Type::Character => colors["color3"],
            Type::Comment | Type::MultilineComment => colors["color6"],
            Type::PrimaryKeywords => colors["color4"],
            Type::SecondaryKeywords => colors["color5"],
            Type::Operators => colors["color9"],
            Type::Brackets => colors["color3"],
            _ => colors["color7"],
        }
    }
}
