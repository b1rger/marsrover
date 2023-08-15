// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

extern crate xdg;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Level {
    pub prob_ditch_one: f64,
    pub prob_ditch_two: f64,
    pub prob_ditch_three: f64,
    pub prob_alien: f64,
    pub prob_alien_jumping: f64,
    pub points: u16,
}

impl Default for Level {
    fn default() -> Self {
        // make random
        Level {
            prob_ditch_one: 0.2,
            prob_ditch_two: 0.0,
            prob_ditch_three: 0.0,
            prob_alien: 0.0,
            prob_alien_jumping: 0.0,
            points: 100,
        }
    }
}

impl Level {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Level {
            prob_ditch_one: rng.gen_range(0.0..0.5),
            prob_ditch_two: rng.gen_range(0.0..0.6),
            prob_ditch_three: rng.gen_range(0.0..0.7),
            prob_alien: rng.gen_range(0.0..0.8),
            prob_alien_jumping: rng.gen_range(0.0..0.9),
            points: rng.gen_range(20..60),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub context_color: Color,
    pub ground_color: Color,
    pub buggy_color: Color,
    pub alien_color: Color,
    pub alien_jumping_color: Color,
    pub bullet_color: Color,
    pub background_color: Color,
    pub levels: Vec<Level>,
}

impl Default for Config {
    fn default() -> Self {
        let l0 = Level::default();
        let l1 = Level {
            prob_ditch_two: 0.2,
            ..Default::default()
        };
        let l2 = Level {
            prob_ditch_two: 0.2,
            prob_ditch_three: 0.2,
            ..Default::default()
        };
        let l3 = Level {
            prob_ditch_two: 0.2,
            prob_ditch_three: 0.2,
            prob_alien: 0.2,
            ..Default::default()
        };
        let l4 = Level {
            prob_ditch_two: 0.2,
            prob_ditch_three: 0.2,
            prob_alien: 0.2,
            prob_alien_jumping: 0.2,
            ..Default::default()
        };
        Config {
            context_color: Color::DarkYellow,
            ground_color: Color::DarkGreen,
            buggy_color: Color::DarkBlue,
            bullet_color: Color::DarkRed,
            alien_color: Color::DarkMagenta,
            alien_jumping_color: Color::DarkCyan,
            background_color: Color::Grey,
            levels: vec![l0, l1, l2, l3, l4],
        }
    }
}

impl Config {
    pub fn read() -> Config {
        match xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            Ok(xdg_dirs) => {
                if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
                    let config_content = fs::read_to_string(&config_path).unwrap_or_default();
                    match toml::from_str(&config_content) {
                        Ok(config) => return config,
                        Err(e) => eprintln!("Could not parse config file: {}", e),
                    }
                } else {
                    //for now disabled, should only be shown with some kind of --debug flag
                    //eprintln!("Could not load configuration file, using default settings.");
                }
            }
            Err(e) => eprintln!("Cannot determine XDG base directories: {}", e),
        }
        Config::default()
    }
}

use crossterm::style::Color as CTC;

impl From<Color> for crossterm::style::Color {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => CTC::Black,
            Color::DarkGrey => CTC::DarkGrey,
            Color::Red => CTC::Red,
            Color::DarkRed => CTC::DarkRed,
            Color::Green => CTC::Green,
            Color::DarkGreen => CTC::DarkGreen,
            Color::Yellow => CTC::Yellow,
            Color::DarkYellow => CTC::DarkYellow,
            Color::Blue => CTC::Blue,
            Color::DarkBlue => CTC::DarkBlue,
            Color::Magenta => CTC::Magenta,
            Color::DarkMagenta => CTC::DarkMagenta,
            Color::Cyan => CTC::Cyan,
            Color::DarkCyan => CTC::DarkCyan,
            Color::White => CTC::White,
            Color::Grey => CTC::Grey,
        }
    }
}
