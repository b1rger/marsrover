// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

extern crate chrono;
extern crate xdg;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

#[derive(Deserialize, Serialize, Debug)]
pub struct Score {
    pub name: String,
    pub points: u16,
    pub timestamp: SystemTime,
}

impl Score {
    pub fn new(name: String, points: u16) -> Self {
        Score {
            name,
            points,
            timestamp: SystemTime::now(),
        }
    }
}

impl From<&Score> for String {
    fn from(score: &Score) -> String {
        let datetime: chrono::DateTime<chrono::Local> = score.timestamp.into();
        format!(
            "{}\t{}:\t\t{} points",
            datetime.format("%d/%m/%Y %T"),
            score.name,
            score.points
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scores {
    pub scores: Vec<Score>,
}

impl From<Scores> for String {
    fn from(scores: Scores) -> String {
        let scores: Vec<String> = scores.scores.iter().map(|score| score.into()).collect();
        scores.join("\n")
    }
}

impl Scores {
    pub fn read() -> Scores {
        if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            if let Some(scores_path) = xdg_dirs.find_state_file("scores.toml") {
                let content = fs::read_to_string(scores_path).unwrap_or_default();
                match toml::from_str(&content) {
                    Ok(scores) => return scores,
                    Err(e) => eprintln!("Could not parse config file: {}", e),
                }
            }
        }
        Scores { scores: vec![] }
    }

    pub fn write(&mut self, max: usize) -> bool {
        self.scores.sort_by_key(|score| score.points);
        self.scores.reverse();
        self.scores.truncate(max);

        if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix(env!("CARGO_CRATE_NAME")) {
            if let Ok(scores_path) = xdg_dirs.place_state_file("scores.toml") {
                if let Ok(scores) = toml::to_string(&self) {
                    if let Ok(mut file) = File::create(scores_path) {
                        return writeln!(&mut file, "{}", scores).is_ok();
                    }
                }
            }
        }
        false
    }
}
