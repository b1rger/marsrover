// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use rand::Rng;

mod alien;
mod buggy;
mod bullet;
mod ditch;

use crate::config;
use alien::Alien;
use buggy::Buggy;
use bullet::Bullet;
use ditch::Ditch;

pub struct World {
    pub cols: u16,
    pub rows: u16,
    pub buggy: Buggy,
    pub bullets: Vec<Bullet>,
    pub aliens: Vec<Alien>,
    pub ditches: Vec<Ditch>,
}

impl Default for World {
    fn default() -> Self {
        World {
            cols: 0,
            rows: 0,
            buggy: Buggy::default(),
            bullets: vec![],
            aliens: vec![],
            ditches: vec![],
        }
    }
}

impl World {
    pub fn shoot(&mut self) {
        self.bullets
            .push(Bullet::new(self.buggy.col + 6, self.buggy.row()));
    }

    pub fn update(&mut self, cols: u16, rows: u16, level: &config::Level) {
        self.cols = cols;
        self.rows = rows;
        self.buggy.update(20, rows - 4);
        self.bullets.iter_mut().for_each(|bullet| bullet.col += 1);

        let mut bulletremovals: Vec<u16> = vec![];
        for bullet in &self.bullets {
            if let Some(pos) = self
                .aliens
                .iter()
                .position(|alien| alien.col == bullet.col && alien.row == bullet.row)
            {
                bulletremovals.push(self.aliens.remove(pos).col)
            }
        }
        self.bullets
            .retain(|bullet| !bulletremovals.contains(&bullet.col));

        if self.buggy.moving() {
            self.ditches.iter_mut().for_each(|hole| hole.col -= 1);
            self.ditches.retain(|hole| hole.col > 0);
            self.aliens.iter_mut().for_each(|alien| alien.col -= 1);
            self.aliens.iter_mut().for_each(|alien| alien.jump());
            self.aliens.retain(|alien| alien.col > 0);

            let mut rng = rand::thread_rng();
            let range = cols - 10..cols;
            if self.aliens.iter().all(|alien| !range.contains(&alien.col))
                && self.ditches.iter().all(|hole| !range.contains(&hole.col))
            {
                if rng.gen_bool(0.5) {
                    if rng.gen_bool(level.prob_ditch_one) {
                        self.ditches.push(Ditch::new(cols, 0));
                    } else if rng.gen_bool(level.prob_ditch_two) {
                        self.ditches.push(Ditch::new(cols, 0));
                        self.ditches.push(Ditch::new(cols + 1, 0));
                    } else if rng.gen_bool(level.prob_ditch_three) {
                        self.ditches.push(Ditch::new(cols, 0));
                        self.ditches.push(Ditch::new(cols + 1, 0));
                        self.ditches.push(Ditch::new(cols + 2, 0));
                    } else if rng.gen_bool(level.prob_alien) {
                        self.aliens.push(Alien::new(cols, rows - 4));
                    } else if rng.gen_bool(level.prob_alien_jumping) {
                        self.aliens.push(Alien::jumping(cols, rows - 4));
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.ditches.clear();
        self.aliens.clear();
    }
}
