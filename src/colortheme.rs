use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use termion::color::Rgb;

pub fn get_colors(term: &str) -> HashMap<String, Rgb> {
    match term {
        "xterm-kitty" | "ansi" => parse_kitty_config("/Users/sergejnehorosev/.config/kitty/current-theme.conf").unwrap(),
        _ => {
            let mut colors = HashMap::new();
            colors.insert("active_border_color".to_owned(), Rgb(0, 0, 0));
            colors.insert("foreground".to_owned(), Rgb(63, 63, 63));
            colors.insert("background".to_owned(), Rgb(253, 253, 253));
            colors.insert("color0".to_owned(), Rgb(220, 163, 163));
            colors.insert("color1".to_owned(), Rgb(38, 139, 210));
            colors.insert("color2".to_owned(), Rgb(41, 174, 26));
            colors.insert("color3".to_owned(), Rgb(108, 113, 196));
            colors.insert("color4".to_owned(), Rgb(181, 137, 0));
            colors.insert("color5".to_owned(), Rgb(42, 161, 152));
            colors.insert("color6".to_owned(), Rgb(133, 153, 0));
            colors.insert("color7".to_owned(), Rgb(255, 255, 255));
            colors.insert("color8".to_owned(), Rgb(85, 85, 85));
            colors.insert("color9".to_owned(), Rgb(220, 163, 163));
            colors.insert("color10".to_owned(), Rgb(38, 139, 210));
            colors.insert("color11".to_owned(), Rgb(41, 174, 26));
            colors.insert("color12".to_owned(), Rgb(108, 113, 196));
            colors.insert("color13".to_owned(), Rgb(181, 137, 0));
            colors.insert("color14".to_owned(), Rgb(42, 161, 152));
            colors.insert("color15".to_owned(), Rgb(133, 153, 0));
            colors
        }
    }
}

fn parse_kitty_config(file_path: &str) -> Result<HashMap<String, Rgb>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut colors = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("color") || line.starts_with("foreground") || line.starts_with("background") || line.starts_with("active_border_color") {
            let mut parts = line.split_whitespace();
            match (parts.nth(0), parts.nth(0)) {
                (Some(color_name), Some(color_value)) => {
                    colors.insert(color_name.to_string(), parse_hex_to_rgb(color_value));
                }
                _ => {}
            }
        }
    }
    Ok(colors)
}

fn parse_hex_to_rgb(hex_code: &str) -> Rgb {
    let red = u8::from_str_radix(&hex_code[1..3], 16).unwrap_or_else(|_| panic!("Invalid red hex code: {}", hex_code));
    let green = u8::from_str_radix(&hex_code[3..5], 16).unwrap_or_else(|_| panic!("Invalid green hex code: {}", hex_code));
    let blue = u8::from_str_radix(&hex_code[5..7], 16).unwrap_or_else(|_| panic!("Invalid blue hex code: {}", hex_code));
    Rgb(red, green, blue)
}