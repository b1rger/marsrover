// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

pub struct Bullet {
    pub col: u16,
    pub row: u16,
}

impl Bullet {
    pub fn new(col: u16, row: u16) -> Self {
        Bullet { col, row }
    }
}
