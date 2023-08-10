// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

pub struct Alien {
    pub col: u16,
    pub row: u16,
}

impl Alien {
    pub fn new(col: u16, row: u16) -> Self {
        Alien { col: col, row: row }
    }
}

impl From<Alien> for String {
    fn from(_alien: Alien) -> String {
        "o".to_string()
    }
}
