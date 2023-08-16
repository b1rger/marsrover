// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use rand::Rng;

mod buggy;
mod bullet;
mod ditch;
mod monster;

use crate::config;
use buggy::Buggy;
use bullet::Bullet;
use ditch::Ditch;
use monster::Monster;

pub struct World {
    pub cols: u16,
    pub rows: u16,
    pub buggy: Buggy,
    pub bullets: Vec<Bullet>,
    pub monsters: Vec<Monster>,
    pub ditches: Vec<Ditch>,
}

impl Default for World {
    fn default() -> Self {
        World {
            cols: 0,
            rows: 0,
            buggy: Buggy::default(),
            bullets: vec![],
            monsters: vec![],
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
            if let Some(pos) = self.monsters.iter().position(|monster| {
                (monster.col == bullet.col || monster.col + 1 == bullet.col)
                    && monster.row == bullet.row
            }) {
                self.monsters.remove(pos);
                bulletremovals.push(bullet.col);
            }
        }
        self.bullets
            .retain(|bullet| !bulletremovals.contains(&bullet.col));

        if self.buggy.moving() {
            self.ditches.iter_mut().for_each(|hole| hole.col -= 1);
            self.ditches.retain(|hole| hole.col > 0);
            self.monsters
                .iter_mut()
                .for_each(|monster| monster.col -= 1);
            self.monsters.iter_mut().for_each(|monster| monster.jump());
            self.monsters.retain(|monster| monster.col > 0);

            let mut rng = rand::thread_rng();
            let range = cols - 10..cols;
            if self
                .monsters
                .iter()
                .all(|monster| !range.contains(&monster.col))
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
                    } else if rng.gen_bool(level.prob_monster) {
                        self.monsters.push(Monster::new(cols, rows - 4));
                    } else if rng.gen_bool(level.prob_monster_jumping) {
                        self.monsters.push(Monster::jumping(cols, rows - 4));
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.ditches.clear();
        self.monsters.clear();
    }
}
