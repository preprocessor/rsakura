use std::{iter, path::PathBuf};

use crossterm::style::Color;

#[derive(clap::Parser, Clone)]
#[command(version, about)]
/// rsakura — terminal cherry blossom screensaver.
///
/// Rust port of the original Nim app: https://github.com/KornelHajto/nsakura
pub struct Cli {
    /// Scales fall speed
    #[clap(short = 's', long = "speed", default_value = "1.0", value_parser = validate_speed)]
    pub speed_factor: f64,

    /// Attached leaf sway amplitude (0.0 to 1.0)
    #[clap(short = 'a', long = "amplitude", default_value = "0.0", value_parser = validate_amplitude)]
    pub sway_amplitude: f64,

    /// Use a custom art file instead of the built-in tree
    #[clap(long = "art", value_parser = clap::value_parser!(PathBuf))]
    pub art_path: Option<PathBuf>,

    /// Use a custom color for the art
    #[clap(long = "art-color", default_value = "White", value_parser = validate_color)]
    pub art_color: Color,

    /// Use a custom color for the leaves
    #[clap(long = "leaf-color", default_value = "Magenta", value_parser = validate_color)]
    pub leaf_color: Color,
}

fn validate_float(s: &str, min: f64, max: f64) -> Result<f64, String> {
    let value: f64 = s.parse().map_err(|_| format!("'{}' isn't a number", s))?;
    if value >= min && value <= max {
        Ok(value as f64)
    } else {
        Err(format!("value not in range {}-{}", min, max))
    }
}

fn validate_speed(s: &str) -> Result<f64, String> {
    validate_float(s, 0., 25.)
}

fn validate_amplitude(s: &str) -> Result<f64, String> {
    validate_float(s, 0., 1.)
}

fn validate_color(s: &str) -> Result<Color, String> {
    let src = s.to_lowercase();

    match src.as_ref() {
        "reset" => Ok(Color::Reset),
        "black" => Ok(Color::Black),
        "dark_grey" => Ok(Color::DarkGrey),
        "red" => Ok(Color::Red),
        "dark_red" => Ok(Color::DarkRed),
        "green" => Ok(Color::Green),
        "dark_green" => Ok(Color::DarkGreen),
        "yellow" => Ok(Color::Yellow),
        "dark_yellow" => Ok(Color::DarkYellow),
        "blue" => Ok(Color::Blue),
        "dark_blue" => Ok(Color::DarkBlue),
        "magenta" => Ok(Color::Magenta),
        "dark_magenta" => Ok(Color::DarkMagenta),
        "cyan" => Ok(Color::Cyan),
        "dark_cyan" => Ok(Color::DarkCyan),
        "white" => Ok(Color::White),
        "grey" => Ok(Color::Grey),
        hex if hex.starts_with('#') => {
            let mut hex: Vec<char> = hex.chars().skip(1).collect();

            for c in &hex {
                match c.to_ascii_lowercase() as u8 {
                    // Convert char to decimal representation
                    48..=57 | 97..=102 => {} // matches 0-9 + a-f in decimal
                    _ => return Err(format!("Invalid color: {}\nBad value: {}", s, c)),
                }
            }

            let len = hex.len();

            if len != 3 && len != 6 {
                return Err(format!("Invalid color: {}\nBad length: {}", s, len));
            }

            if len == 3 {
                // Double each char, abc -> aabbcc
                hex = hex.iter().flat_map(|&c| iter::repeat_n(c, 2)).collect();
            }

            let rgb: Vec<u8> = [0, 2, 4]
                .iter()
                .map(|&i| {
                    let c = format!("{}{}", hex[i], hex[i + 1]);
                    u8::from_str_radix(&c, 16)
                        .map_err(|_| format!("Invalid color: {}\nBad value: {}", s, c))
                })
                .collect::<Result<_, _>>()?;

            let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

            return Ok(Color::Rgb { r, g, b });
        }
        rgb => {
            let rgb: Vec<u8> = rgb
                .splitn(3, ',')
                .map(|v| v.parse().map_err(|_| format!("Invalid color: {}", s)))
                .collect::<Result<_, _>>()?;

            if rgb.len() != 3 {
                return Err(format!("Invalid color: {}", s));
            }

            let (r, g, b) = (rgb[0], rgb[1], rgb[2]);
            Ok(Color::from((r, g, b)))
        }
    }
}
