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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Level {
    pub prob_crater_one: f64,
    pub prob_crater_two: f64,
    pub prob_crater_three: f64,
    pub prob_monster: f64,
    pub prob_monster_jumping: f64,
    pub points: u16,
    pub desc: String,
}

impl Default for Level {
    fn default() -> Self {
        // make random
        Level {
            prob_crater_one: 0.2,
            prob_crater_two: 0.0,
            prob_crater_three: 0.0,
            prob_monster: 0.0,
            prob_monster_jumping: 0.0,
            points: 100,
            desc: String::default(),
        }
    }
}

impl Level {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Level {
            prob_crater_one: rng.gen_range(0.0..0.5),
            prob_crater_two: rng.gen_range(0.0..0.6),
            prob_crater_three: rng.gen_range(0.0..0.7),
            prob_monster: rng.gen_range(0.0..0.8),
            prob_monster_jumping: rng.gen_range(0.0..0.9),
            points: rng.gen_range(20..60),
            desc: String::from("Wohoo! This level is completely random!"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Config {
    pub color_context: Color,
    pub color_ground: Color,
    pub char_ground: char,
    pub color_rover: Color,
    pub color_monster: Color,
    pub char_monster: char,
    pub color_monster_jumping: Color,
    pub char_monster_jumping: char,
    pub color_bullet: Color,
    pub char_bullet: char,
    pub color_background: Color,
    pub levels: Vec<Level>,
}

impl Default for Config {
    fn default() -> Self {
        let l0 = Level::default();
        let l1 = Level {
            prob_crater_two: 0.2,
            desc: String::from("In level one the craters are a bit wider!"),
            ..Default::default()
        };
        let l2 = Level {
            prob_crater_two: 0.2,
            prob_crater_three: 0.2,
            desc: String::from("In level two the craters can be three wide!"),
            ..Default::default()
        };
        let l3 = Level {
            prob_crater_two: 0.2,
            prob_crater_three: 0.2,
            prob_monster: 0.2,
            desc: String::from("Oh now! There might be monsters. Shoot them!"),
            ..Default::default()
        };
        let l4 = Level {
            prob_crater_two: 0.2,
            prob_crater_three: 0.2,
            prob_monster: 0.2,
            prob_monster_jumping: 0.2,
            desc: String::from("Some monsters are jumping to evade the bullets!"),
            ..Default::default()
        };
        Config {
            color_context: Color::DarkYellow,
            color_ground: Color::DarkGreen,
            char_ground: '#',
            color_rover: Color::DarkBlue,
            color_bullet: Color::DarkRed,
            char_bullet: '-',
            color_monster: Color::DarkMagenta,
            char_monster: 'o',
            color_monster_jumping: Color::DarkCyan,
            char_monster_jumping: 'O',
            color_background: Color::DarkYellow,
            levels: vec![l0, l1, l2, l3, l4],
        }
    }
}

impl Config {
    pub fn read() -> Config {
        match xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            Ok(xdg_dirs) => {
                if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
                    let config_content = fs::read_to_string(config_path).unwrap_or_default();
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
