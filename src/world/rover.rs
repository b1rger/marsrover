// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

#[derive(PartialEq, Clone, Copy)]
enum RoverState {
    Run,
    Jump(u32),
    Crash(u32),
    Monster(u32),
}

#[derive(Clone, Copy)]
pub struct Rover {
    pub col: u16,
    pub row: u16,
    state: RoverState,
    pub lives: u16,
    pub points: u16,
    pub tick: usize,
}

impl Default for Rover {
    fn default() -> Self {
        Rover {
            col: 20,
            row: 0,
            state: RoverState::Run,
            lives: 3,
            points: 0,
            tick: 0,
        }
    }
}

impl Rover {
    pub fn update(&mut self, col: u16, row: u16) {
        self.col = col;
        self.row = row;
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.tick %= 16;
        if let RoverState::Jump(x) = self.state {
            if x <= 8 {
                self.state = RoverState::Jump(x + 1);
            } else {
                self.state = RoverState::Run;
            }
        }
        if let RoverState::Crash(x) = self.state {
            if x < 32 {
                self.state = RoverState::Crash(x + 1)
            } else {
                self.state = RoverState::Run
            }
        }
        if let RoverState::Monster(x) = self.state {
            if x < 32 {
                self.state = RoverState::Monster(x + 1)
            } else {
                self.state = RoverState::Run
            }
        }
    }

    pub fn moving(&mut self) -> bool {
        if let RoverState::Jump(_) = self.state {
            return true;
        }
        self.state == RoverState::Run
    }

    pub fn rebooting(&mut self) -> bool {
        RoverState::Crash(32) == self.state || RoverState::Monster(32) == self.state
    }

    pub fn jump(&mut self) {
        if self.state == RoverState::Run {
            self.state = RoverState::Jump(0);
            return;
        }
        self.state = RoverState::Run;
    }

    pub fn crash(&mut self) {
        self.state = RoverState::Crash(0);
        self.lives -= 1;
    }

    pub fn monstercrash(&mut self) {
        self.state = RoverState::Monster(0);
        self.lives -= 1;
    }

    pub fn jumping(&self) -> bool {
        if let RoverState::Jump(_) = self.state {
            return true;
        }
        false
    }

    pub fn row(&self) -> u16 {
        match self.state {
            RoverState::Jump(x) => match x {
                1..=6 => self.row - 2,
                _ => self.row - 1,
            },
            _ => self.row,
        }
    }

    pub fn range(&self) -> std::ops::Range<u16> {
        self.col + 1..self.col + 5
    }
}

impl From<Rover> for String {
    fn from(rover: Rover) -> String {
        let body: &str = " mm0";
        let tire: &str = match rover.tick % 4 {
            0 => "(|)",
            1 => "(/)",
            2 => "(-)",
            _ => "(\\)",
        };
        let exhaust: &str = match rover.tick {
            0 => "   .",
            1 => "  o.",
            2 => " oo ",
            3 => "0o  ",
            _ => "    ",
        };
        match rover.state {
            RoverState::Run => format!("{exhaust}{body:width$}\n    {tire}-{tire}", width = 10),
            RoverState::Jump(_) => format!(
                "{exhaust}{body:width$}\n    {tire}-{tire:width$}",
                width = 10
            ),
            RoverState::Crash(x) => match x {
                0..=5 => format!(
                    "{:width$}\ncnOMMnb{:>xwidth$}",
                    "_",
                    "o",
                    width = 10,
                    xwidth = x as usize
                ),
                _ => format!(
                    "{:width$}\ncnOMMnb{:>xwidth$}",
                    "_",
                    "_",
                    width = 10,
                    xwidth = 6 as usize
                ),
            },
            RoverState::Monster(x) => match x {
                0..=2 => format!("(_) mm0(_)"),
                3..=5 => format!("(o)/mm0(o)?"),
                6..=8 => format!(".o)_mm0(o.??"),
                _ => format!(".o).mm0(o.???"),
            },
        }
    }
}
