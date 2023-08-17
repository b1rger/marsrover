// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

pub struct Monster {
    pub col: u16,
    pub row: u16,
    pub jumping: Option<i32>,
}

impl Monster {
    pub fn new(col: u16, row: u16) -> Self {
        Monster {
            col: col,
            row: row,
            jumping: None,
        }
    }
    pub fn jumping(col: u16, row: u16) -> Self {
        Monster {
            col: col,
            row: row,
            jumping: Some(0),
        }
    }
    pub fn jump(&mut self) {
        if let Some(x) = self.jumping {
            match x {
                0..=1 => {
                    self.jumping = Some(x + 1);
                    self.row -= 1;
                }
                2 => self.jumping = Some(-x),
                _ => {
                    self.jumping = Some(x + 1);
                    self.row += 1;
                }
            }
        }
    }
}
