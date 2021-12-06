/*
 * Copyright 2021 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fs;
use std::string::String;
use std::str::FromStr;
use std::convert::TryInto;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Game {
    nums: Vec<isize>,
    boards: Vec<[isize; 25]>,
    state: Vec<[bool; 25]>,
    just_called: Option<isize>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = s.find('\n').ok_or("Couldn't find newline")?;
        let (nums_string, boards_string) = s.split_at(idx);

        let nums = nums_string.split(',').map(|n| n.parse())
            .rev().collect::<Result<_,_>>()
            .or(Err("Couldn't parse move list"))?;

        let boards =
            boards_string.split_terminator("\n\n").skip(1).map(|board| {
                board.split_whitespace().map(|n| n.parse())
                    .collect::<Result<Vec<_>,_>>().or(
                        Err("Board contained non-integer")
                    )?.try_into().or(
                        Err("Board doesn't have 25 squares")
                    )
            }).collect::<Result<Vec<_>, _>>()?;

        let mut state = Vec::new();
        for _ in 0..boards.len() {
            state.push([false; 25])
        }

        Ok(Game { nums, boards, state, just_called : None })
    }
}

impl Game {
    fn run_step(&mut self) -> Result<(), &'static str> {
        let n = self.nums.pop().ok_or("Out of moves!")?;

        self.just_called = Some(n);

        for (board_idx, board) in self.boards.iter().enumerate() {
            for (sq_idx, sq) in board.iter().enumerate() {
                if *sq == n {
                    self.state[board_idx][sq_idx] = true;
                }
            }
        }

        Ok(())
    }

    fn is_winning(&self, idx: usize) -> bool {
        'outer1: for row in 0..5 {
            for col in 0..5 {
                if !self.state[idx][row*5 + col] {
                    continue 'outer1
                }
            }
            return true;
        }

        'outer2: for col in 0..5 {
            for row in 0..5 {
                if !self.state[idx][row*5 + col] {
                    continue 'outer2
                }
            }
            return true;
        }
        false
    }

    fn winning_board(&self) -> Option<usize> {
        for board_idx in 0..self.boards.len() {
            if self.is_winning(board_idx) {
                return Some(board_idx)
            }
        }
        None
    }

    fn loosing_board(&self) -> Option<usize> {
        for board_idx in 0..self.boards.len() {
            if !self.is_winning(board_idx) {
                return Some(board_idx)
            }
        }
        None
    }

    fn board_score(&self, idx: usize) -> isize {
        let mut score = 0;
        for i in 0..25 {
            if self.state[idx][i] {
                continue
            };

            score += self.boards[idx][i];
        }
        score
    }
}

fn first_part(s: &str) -> isize {
    let mut g : Game = s.parse().expect("Couldn't parse game");

    let board_score = loop {
        g.run_step().expect("Failed to run step");
        match g.winning_board() {
            Some(idx) => break g.board_score(idx),
            None => (),
        };
    };

    board_score * g.just_called.unwrap()
}

fn second_part(s: &str) -> isize {
    let mut g : Game = s.parse().expect("Couldn't parse game");

    let mut idx = 0;
    let board_score = loop {
        g.run_step().expect("Failed to run step");
        match g.loosing_board() {
            Some(new_idx) => idx = new_idx,
            None => break g.board_score(idx),
        };
    };

    board_score * g.just_called.unwrap()
}
