// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crossterm::style::Stylize;
use std::fmt;

use crate::config::Config;
use crate::world::World;

#[derive(PartialEq)]
pub enum State {
    Run,
    Quit,
}

pub struct Message {
    pub message: String,
    pub tick: u16,
}

pub struct Context {
    pub state: State,
    pub config: Config,
    pub level: usize,
    pub messages: Vec<Message>,
    pub world: World,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            state: State::Run,
            config: Config::read(),
            level: 0,
            messages: vec![],
            world: World::default(),
        }
    }
}

impl Context {
    pub fn run(&self) -> bool {
        self.state == State::Run && self.world.buggy.lives > 0
    }
    pub fn quit(&mut self) {
        self.state = State::Quit;
    }
    pub fn addmessage(&mut self, message: String, tick: u16) {
        let msg = Message { message, tick };
        self.messages.push(msg);
    }

    pub fn worldtolines(&self) -> Vec<String> {
        let rows = self.world.rows.into();
        let cols = self.world.cols.into();
        let mut worldlines = vec![vec![' '.to_string(); cols]; rows];

        worldlines[rows - 2] = vec![
            self.config
                .char_ground
                .to_string()
                .with(self.config.color_ground.into())
                .to_string();
            cols
        ];
        worldlines[rows - 3] = vec![
            self.config
                .char_ground
                .to_string()
                .with(self.config.color_ground.into())
                .to_string();
            cols
        ];
        for ditch in &self.world.ditches {
            if (ditch.col as usize) < cols && (ditch.row as usize) < rows {
                worldlines[rows - 3][ditch.col as usize] = ' '.to_string();
            }
        }
        let monsters = self
            .world
            .monsters
            .iter()
            .filter(|monster| (monster.col as usize) < cols && (monster.row as usize) < rows);
        for monster in monsters {
            if monster.jumping.is_some() {
                worldlines[monster.row as usize][monster.col as usize] = self
                    .config
                    .char_monster_jumping
                    .to_string()
                    .with(self.config.color_monster_jumping.into())
                    .to_string();
            } else {
                worldlines[monster.row as usize][monster.col as usize] = self
                    .config
                    .char_monster
                    .to_string()
                    .with(self.config.color_monster.into())
                    .to_string();
            }
        }
        for bullet in &self.world.bullets {
            if (bullet.col as usize) < cols && (bullet.row as usize) < rows {
                worldlines[bullet.row as usize][bullet.col as usize] = self
                    .config
                    .char_bullet
                    .to_string()
                    .with(self.config.color_bullet.into())
                    .to_string();
            }
        }
        worldlines.iter().map(|rowvec| rowvec.join("")).collect()
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lives: {} Points: {}, Level: {}/{}",
            self.world.buggy.lives,
            self.world.buggy.points,
            self.level,
            self.config.levels.len()
        )
    }
}
