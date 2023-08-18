// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use std::io;
use std::time::Duration;

use crate::context;

pub fn events(ctx: &mut context::Context) -> io::Result<()> {
    if poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(' ') => ctx.world.rover.jump(),
                KeyCode::Char('j') => ctx.world.shoot(),
                KeyCode::Char('q') => ctx.quit(),
                _ => (),
            }
        }
    }
    Ok(())
}

pub fn read_name(line: &mut String) -> bool {
    if poll(Duration::from_millis(100)).is_ok() {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Enter => {
                    return false;
                }
                KeyCode::Char(c) => {
                    line.push(c);
                }
                _ => {}
            }
        }
    }
    true
}
