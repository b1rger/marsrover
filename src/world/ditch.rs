// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

pub struct Ditch {
    pub col: u16,
    pub row: u16,
}

impl Ditch {
    pub fn new(col: u16, row: u16) -> Self {
        Ditch { col: col, row: row }
    }
}
