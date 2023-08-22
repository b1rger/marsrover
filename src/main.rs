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

    // if user requests the scoreboard using the `-s` switch,
    // just print that and exit gracefully
    if let Some(arg) = std::env::args().nth(1) {
        if arg.eq("-s") {
            let scores: String = scores::Scores::read().into();
            println!("Marsrover-Scores\n{}", scores);
            std::process::exit(0);
        }
    }

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
        if ctx.world.rover.moving() {
            if ctx
                .world
                .craters
                .iter()
                .any(|hole| hole.col == ctx.world.rover.col)
            {
                ctx.world.rover.points += 4;
            }
            if ctx
                .world
                .craters
                .iter()
                .any(|hole| ctx.world.rover.range().contains(&hole.col))
                && !ctx.world.rover.jumping()
            {
                ctx.world.rover.crash();
            }
            if ctx
                .world
                .monsters
                .iter()
                .any(|monster| monster.col == ctx.world.rover.col + 5)
            {
                ctx.world.rover.monstercrash();
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

        /* draw the rover */
        let bstr: String = ctx.world.rover.into();
        for (index, line) in bstr.lines().rev().enumerate() {
            draw(
                &stdout,
                ctx.world.rover.col,
                ctx.world.rover.row() - index as u16,
                line.to_string(),
                ctx.config.color_rover,
            )?;
        }
        ctx.world.rover.tick();

        // sum up the points of all levels up to now...
        let points: u16 = ctx.config.levels[0..=ctx.level]
            .iter()
            .map(|x| x.points)
            .sum();
        if points <= ctx.world.rover.points {
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
        for (index, message) in ctx.messages.iter_mut().rev().enumerate() {
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

        if ctx.world.rover.rebooting() {
            ctx.world.reset();
        }

        stdout.flush()?;
    }

    if ctx.world.rover.points > 0 {
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
                .push(scores::Score::new(name, ctx.world.rover.points));
            scores.write(10);
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
