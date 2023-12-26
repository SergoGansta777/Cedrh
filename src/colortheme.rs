use shellexpand::tilde;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;

use crossterm::style::Color;

const CUSTOM_CONFIG_PATH: &str = "~/.config/cedrh/cedrh.conf";

#[must_use]
pub fn get_colors(term: &str) -> HashMap<String, Color> {
    if let Ok(custom_config) = parse_simple_config(CUSTOM_CONFIG_PATH) {
        return custom_config;
    }
    match term {
        "xterm-kitty" | "ansi" => parse_simple_config("~/.config/kitty/current-theme.conf")
            .unwrap_or(
                parse_simple_config("`/.config/kitty/kitty.conf`").unwrap_or(get_default_colors()),
            ),
        _ => get_default_colors(),
    }
}
#[allow(clippy::too_many_lines)]
fn get_default_colors() -> HashMap<String, Color> {
    let mut colors = HashMap::new();
    colors.insert(
        "active_border_color".to_owned(),
        Color::Rgb {
            r: 172,
            g: 170,
            b: 255,
        },
    );
    colors.insert("foreground".to_owned(), Color::Rgb { r: 6, g: 5, b: 22 });
    colors.insert("background".to_owned(), Color::Rgb { r: 6, g: 5, b: 22 });
    colors.insert(
        "color0".to_owned(),
        Color::Rgb {
            r: 0,
            g: 194,
            b: 186,
        },
    );
    colors.insert(
        "color1".to_owned(),
        Color::Rgb {
            r: 68,
            g: 168,
            b: 231,
        },
    );
    colors.insert(
        "color2".to_owned(),
        Color::Rgb {
            r: 166,
            g: 227,
            b: 161,
        },
    );
    colors.insert(
        "color3".to_owned(),
        Color::Rgb {
            r: 172,
            g: 170,
            b: 255,
        },
    );
    colors.insert(
        "color4".to_owned(),
        Color::Rgb {
            r: 171,
            g: 139,
            b: 227,
        },
    );
    colors.insert(
        "color5".to_owned(),
        Color::Rgb {
            r: 243,
            g: 139,
            b: 168,
        },
    );
    colors.insert(
        "color6".to_owned(),
        Color::Rgb {
            r: 249,
            g: 226,
            b: 175,
        },
    );
    colors.insert(
        "color7".to_owned(),
        Color::Rgb {
            r: 186,
            g: 194,
            b: 222,
        },
    );
    colors.insert(
        "color8".to_owned(),
        Color::Rgb {
            r: 85,
            g: 85,
            b: 85,
        },
    );
    colors.insert(
        "color9".to_owned(),
        Color::Rgb {
            r: 220,
            g: 163,
            b: 163,
        },
    );
    colors.insert(
        "color10".to_owned(),
        Color::Rgb {
            r: 38,
            g: 139,
            b: 210,
        },
    );
    colors.insert(
        "color11".to_owned(),
        Color::Rgb {
            r: 41,
            g: 174,
            b: 26,
        },
    );
    colors.insert(
        "color12".to_owned(),
        Color::Rgb {
            r: 108,
            g: 113,
            b: 196,
        },
    );
    colors.insert(
        "color13".to_owned(),
        Color::Rgb {
            r: 181,
            g: 137,
            b: 0,
        },
    );
    colors.insert(
        "color14".to_owned(),
        Color::Rgb {
            r: 42,
            g: 161,
            b: 152,
        },
    );
    colors.insert(
        "color15".to_owned(),
        Color::Rgb {
            r: 133,
            g: 153,
            b: 0,
        },
    );
    colors
}

fn parse_simple_config(config_path: &str) -> Result<HashMap<String, Color>> {
    let expanded_path = tilde(config_path);
    let path = PathBuf::from(expanded_path.into_owned());
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut colors = HashMap::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("color")
            || line.starts_with("foreground")
            || line.starts_with("background")
            || line.starts_with("active_border_color")
        {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(color_name), Some(color_value)) => {
                    colors.insert(color_name.to_owned(), parse_hex_to_rgb(color_value));
                }
                _ => {}
            }
        }
    }
    Ok(colors)
}

#[allow(clippy::indexing_slicing, clippy::string_slice)]
fn parse_hex_to_rgb(hex_code: &str) -> Color {
    let red = u8::from_str_radix(&hex_code[1..3], 16)
        .unwrap_or_else(|_| panic!("Invalid red hex code: {hex_code}"));
    let green = u8::from_str_radix(&hex_code[3..5], 16)
        .unwrap_or_else(|_| panic!("Invalid green hex code: {hex_code}"));
    let blue = u8::from_str_radix(&hex_code[5..7], 16)
        .unwrap_or_else(|_| panic!("Invalid blue hex code: {hex_code}"));
    Color::Rgb {
        r: red,
        g: green,
        b: blue,
    }
}
