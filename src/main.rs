// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    terminal,
    terminal::size,
    ExecutableCommand,
};
use std::io::{self, Write};

mod config;
mod context;
mod events;
mod scores;
mod world;

use context::Context;

fn draw(
    mut stdout: &std::io::Stdout,
    col: u16,
    row: u16,
    text: String,
    color: config::Color,
) -> io::Result<()> {
    queue!(
        stdout,
        cursor::MoveTo(col, row),
        style::PrintStyledContent(text.with(color.into()))
    )?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut ctx = Context::default();

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::Hide)?;

    while ctx.run() {
        let (cols, rows) = size()?;
        ctx.world.update(cols, rows, &ctx.config.levels[ctx.level]);

        events::events(&mut ctx)?;

        queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

        draw(
            &stdout,
            0,
            rows - 1,
            format!("{}", ctx),
            ctx.config.context_color,
        )?;

        /* draw the ground below */
        let belowground = vec!['#'; cols as usize].iter().cloned().collect::<String>();
        draw(&stdout, 0, rows - 2, belowground, ctx.config.ground_color)?;

        /* calculate actions */
        if ctx.world.buggy.moving() {
            if ctx
                .world
                .ditches
                .iter()
                .any(|hole| hole.col == ctx.world.buggy.col)
            {
                ctx.world.buggy.points += 4;
            }
            if ctx
                .world
                .ditches
                .iter()
                .any(|hole| ctx.world.buggy.range().contains(&hole.col))
                && !ctx.world.buggy.jumping()
            {
                ctx.world.buggy.crash();
            }
            if ctx
                .world
                .aliens
                .iter()
                .any(|alien| alien.col == ctx.world.buggy.col + 5)
            {
                ctx.world.buggy.alienate();
            }
        }

        /* draw the ground */
        let worldstr: String = ctx.world.to_string();
        for (index, line) in worldstr.lines().enumerate() {
            draw(
                &stdout,
                0,
                rows - 3 - index as u16,
                line.to_string(),
                ctx.config.ground_color,
            )?;
        }

        /* draw bullets */
        for bullet in &ctx.world.bullets {
            draw(
                &stdout,
                bullet.col,
                bullet.row,
                "-".to_string(),
                ctx.config.bullet_color,
            )?;
        }

        /* draw the buggy */
        let bstr: String = ctx.world.buggy.into();
        for (index, line) in bstr.lines().rev().enumerate() {
            draw(
                &stdout,
                ctx.world.buggy.col,
                ctx.world.buggy.row() - index as u16,
                line.to_string(),
                ctx.config.buggy_color,
            )?;
        }
        ctx.world.buggy.tick();

        // sum up the points of all levels up to now...
        let points: u16 = ctx.config.levels[0..=ctx.level]
            .iter()
            .map(|x| x.points)
            .sum();
        if points <= ctx.world.buggy.points {
            if ctx.level < ctx.config.levels.len() - 1 {
                ctx.level += 1;
                ctx.addmessage(format!("Level up! You're now on level {}", ctx.level), 40);
            } else {
                ctx.config.levels.push(config::Level::random());
            }
        }

        // print messages, if any
        ctx.messages.retain(|message| message.tick > 0);
        for (index, mut message) in ctx.messages.iter_mut().enumerate() {
            message.tick -= 1;
            let pos: u16 = rows - 20 - index as u16;
            if message.tick > 0 {
                draw(
                    &stdout,
                    20,
                    pos,
                    message.message.to_string(),
                    config::Color::White,
                )?;
            }
        }

        if ctx.world.buggy.rebooting() {
            ctx.world.reset();
        }

        stdout.flush()?;
    }

    if ctx.world.buggy.points > 0 {
        let mut scores = scores::Scores::read();

        draw(
            &stdout,
            10,
            2,
            "Scoreboard:".to_string(),
            config::Color::White,
        )?;
        for (index, score) in scores.scores.iter().enumerate() {
            draw(
                &stdout,
                10,
                3 + index as u16,
                score.into(),
                config::Color::White,
            )?;
        }
        stdout.flush()?;

        let (_cols, rows) = size()?;
        queue!(
            stdout,
            cursor::MoveTo(0, rows - 1),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;

        draw(
            &stdout,
            0,
            rows - 1,
            "Enter your name:".to_string(),
            config::Color::White,
        )?;
        stdout.flush()?;

        let mut name = String::new();
        while events::read_name(&mut name) {
            draw(
                &stdout,
                0,
                rows - 1,
                format!("Enter your name: {}", name),
                config::Color::White,
            )?;
            stdout.flush()?;
        }

        if !name.is_empty() {
            scores
                .scores
                .push(scores::Score::new(name, ctx.world.buggy.points));
            scores.write(10);
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}