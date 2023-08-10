// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

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
