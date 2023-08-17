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
            ctx.config.color_context,
        )?;

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
                .monsters
                .iter()
                .any(|monster| monster.col == ctx.world.buggy.col + 5)
            {
                ctx.world.buggy.monstercrash();
            }
        }

        for (row, line) in ctx.worldtolines().iter().enumerate() {
            // only draw if there is actually anything to draw
            if line.chars().any(|c| c != ' ') {
                draw(
                    &stdout,
                    0,
                    row as u16,
                    line.to_string(),
                    config::Color::White,
                )?;
            }
        }

        /* draw the buggy */
        let bstr: String = ctx.world.buggy.into();
        for (index, line) in bstr.lines().rev().enumerate() {
            draw(
                &stdout,
                ctx.world.buggy.col,
                ctx.world.buggy.row() - index as u16,
                line.to_string(),
                ctx.config.color_buggy,
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
                ctx.addmessage(ctx.config.levels[ctx.level].desc.to_string(), 40);
            } else {
                ctx.config.levels.push(config::Level::random());
            }
        }

        // print messages, if any
        ctx.messages.retain(|message| message.tick > 0);
        for (index, mut message) in ctx.messages.iter_mut().rev().enumerate() {
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
