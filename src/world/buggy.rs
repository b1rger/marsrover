// SPDX-FileCopyrightText: 2023 Birger Schacht <birger@rantanplan.org>
//
// SPDX-License-Identifier: MIT

#[derive(Clone, Copy)]
enum Tire {
    P1,
    P2,
    P3,
    P4,
}

impl Tire {
    pub fn right(&mut self) {
        *self = match *self {
            Tire::P1 => Tire::P2,
            Tire::P2 => Tire::P3,
            Tire::P3 => Tire::P4,
            Tire::P4 => Tire::P1,
        };
    }
}

#[derive(PartialEq, Clone, Copy)]
enum BuggyState {
    Run,
    Jump(u32),
    Crash(u32),
    Monster(u32),
}

#[derive(Clone, Copy)]
pub struct Buggy {
    pub col: u16,
    pub row: u16,
    tire1: Tire,
    tire2: Tire,
    state: BuggyState,
    pub lives: u16,
    pub points: u16,
}

impl Default for Buggy {
    fn default() -> Self {
        Buggy {
            col: 20,
            row: 0,
            tire1: Tire::P1,
            tire2: Tire::P1,
            state: BuggyState::Run,
            lives: 3,
            points: 0,
        }
    }
}

impl Buggy {
    pub fn update(&mut self, col: u16, row: u16) {
        self.col = col;
        self.row = row;
    }

    pub fn tick(&mut self) {
        if let BuggyState::Jump(x) = self.state {
            if x <= 8 {
                self.state = BuggyState::Jump(x + 1);
            } else {
                self.state = BuggyState::Run;
            }
        }
        if let BuggyState::Crash(x) = self.state {
            if x < 32 {
                self.state = BuggyState::Crash(x + 1)
            } else {
                self.state = BuggyState::Run
            }
        }
        if let BuggyState::Monster(x) = self.state {
            if x < 32 {
                self.state = BuggyState::Monster(x + 1)
            } else {
                self.state = BuggyState::Run
            }
        }

        self.tire1.right();
        self.tire2.right();
    }

    pub fn right(&mut self) {
        self.tire1.right();
        self.tire2.right();
    }

    pub fn moving(&mut self) -> bool {
        if let BuggyState::Jump(_) = self.state {
            return true;
        }
        self.state == BuggyState::Run
    }

    pub fn rebooting(&mut self) -> bool {
        BuggyState::Crash(32) == self.state || BuggyState::Monster(32) == self.state
    }

    pub fn jump(&mut self) {
        if self.state == BuggyState::Run {
            self.state = BuggyState::Jump(0);
            return;
        }
        self.state = BuggyState::Run;
    }

    pub fn crash(&mut self) {
        self.state = BuggyState::Crash(0);
        self.lives -= 1;
    }

    pub fn monstercrash(&mut self) {
        self.state = BuggyState::Monster(0);
        self.lives -= 1;
    }

    pub fn jumping(&self) -> bool {
        if let BuggyState::Jump(_) = self.state {
            return true;
        }
        false
    }

    pub fn row(&self) -> u16 {
        match self.state {
            BuggyState::Jump(x) => match x {
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

impl From<Buggy> for String {
    fn from(buggy: Buggy) -> String {
        let body: String = " mm0".to_string();
        let tire1: String = buggy.tire1.into();
        let tire2: String = buggy.tire2.into();
        match buggy.state {
            BuggyState::Run => format!("{body:width$}\n{tire1}-{tire2}", width = 10),
            BuggyState::Jump(_) => format!("{body:width$}\n{tire1}-{tire2:width$}", width = 10),
            BuggyState::Crash(x) => match x {
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
            BuggyState::Monster(x) => match x {
                0..=2 => format!("(_) mm0(_)"),
                3..=5 => format!("(o)/mm0(o)?"),
                6..=8 => format!(".o)_mm0(o.??"),
                _ => format!(".o).mm0(o.???"),
            },
        }
    }
}

impl From<Tire> for String {
    fn from(t: Tire) -> String {
        match t {
            Tire::P1 => "(|)".to_string(),
            Tire::P2 => "(/)".to_string(),
            Tire::P3 => "(-)".to_string(),
            Tire::P4 => "(\\)".to_string(),
        }
    }
}
