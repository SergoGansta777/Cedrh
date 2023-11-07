use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::color::Rgb;

pub fn get_colors(term: &str) -> HashMap<String, Rgb> {
    match term {
        "xterm-kitty" | "ansi" => parse_kitty_config("/Users/sergejnehorosev/.config/kitty/current-theme.conf"),
        _ => {
            let mut colors = HashMap::new();
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

fn parse_kitty_config(file_path: &str) -> HashMap<String, Rgb> {
    let file = File::open(file_path).expect("failed to open kitty config file");
    let reader = BufReader::new(file);

    let mut colors = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line in kitty config");

        if line.starts_with("color") {
            let mut parts = line.split_whitespace();
            if let Some(color_name) = parts.nth(0) {
                if let Some(color_value) = parts.nth(0) {
                    colors.insert(color_name.to_string(), parse_hex_to_rgb(color_value));
                }
            }
        }
    }
    colors
}

fn parse_hex_to_rgb(hex_code: &str) -> Rgb {
    let red = u8::from_str_radix(&hex_code[1..3], 16).unwrap();
    let green = u8::from_str_radix(&hex_code[3..5], 16).unwrap();
    let blue = u8::from_str_radix(&hex_code[5..7], 16).unwrap();
    Rgb(red, green, blue)
}
