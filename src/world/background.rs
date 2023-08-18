// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use rand::Rng;
use rand::seq::SliceRandom;

pub struct Background {
    pub col: u16,
    pub row: u16,
    pub chr: char,
}

impl Background {
    pub fn new(cols: u16, rows: u16) -> Option<Self> {
        let mut rng = rand::thread_rng();
        let chars = vec!['*', '+'];
        match chars.choose(&mut rng) {
            Some(x) => Some(Background {
                col: cols,
                row: rng.gen_range(0..rows-10),
                chr: *x,
            }),
            _ => None,
        }
    }
}
