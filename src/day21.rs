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
use itertools::Itertools;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

fn parse(s: &str) -> (u8,u8) {
    let (line1, line2) = s.split_once('\n').unwrap();

    let p1 : u8 = line1.split_whitespace()
        .next_back().unwrap().parse().unwrap();
    let p2 : u8 = line2.split_whitespace()
        .next_back().unwrap().parse().unwrap();

    (p1-1, p2-1)
}

struct GameResult {
    turns: usize,
    p1_wins: bool,
    score: (usize, usize),
}

fn get_roll<I>(it: &mut I) -> u8
    where I : Iterator<Item=u8> {
    it.next().unwrap() % 10 +
        it.next().unwrap() % 10 +
        it.next().unwrap() % 10
}

fn run_game<I>(start: (u8,u8), mut it: I) -> GameResult
    where I : Iterator<Item=u8> {

    let mut score = (0,0);
    let mut pos = start;
    let mut turns = 0;

    let p1_wins = loop {
        let p1_roll = get_roll(&mut it);
        pos.0 += p1_roll;
        pos.0 %= 10;
        score.0 += pos.0 as usize + 1;
        turns += 1;

        if score.0 >= 1000 {
            break true;
        }

        let p2_roll = get_roll(&mut it);
        pos.1 += p2_roll;
        pos.1 %= 10;
        score.1 += pos.1 as usize + 1;
        turns += 1;

        if score.1 >= 1000 {
            break false;
        }
    };

    GameResult { turns, p1_wins, score }
}

fn first_part(s: &str) -> usize {
    let start = parse(s);
    let result = run_game(start, (1..101).cycle());

    if result.p1_wins {
        result.score.1 * result.turns * 3
    } else {
        result.score.0 * result.turns * 3
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct GameState {
    p1_turn: bool,
    score: (u8, u8),
    pos: (u8,u8),
}

fn second_part(s: &str) -> u64 {
    let start = parse(s);

    let init_state = GameState {
        p1_turn: true,
        score: (0,0),
        pos: start,
    };

    let dice_combo = vec![
        (3,1),
        (4,3),
        (5,6),
        (6,7),
        (7,6),
        (8,3),
        (9,1),
    ];

    let mut map = HashMap::new();
    map.insert(init_state, 1u64);

    for ((((a,b),c),d),e) in (0..21)
        .cartesian_product(0..21)
        .cartesian_product(0..10)
        .cartesian_product(0..10)
        .cartesian_product([false, true])
    {
        let curr_state = GameState {
            score: (a, b),
            pos: (c, d),
            p1_turn: e,
        };
        if let Some(count) = map.remove(&curr_state) {
            let mut new_state = curr_state;
            new_state.p1_turn = !new_state.p1_turn;
            for (n,c) in &dice_combo {
                if curr_state.p1_turn {
                    new_state.pos.0 = (curr_state.pos.0 + n) % 10;
                    new_state.score.0 = curr_state.score.0 + new_state.pos.0 + 1;
                } else {
                    new_state.pos.1 = (curr_state.pos.1 + n) % 10;
                    new_state.score.1 = curr_state.score.1 + new_state.pos.1 + 1;               }
                *map.entry(new_state).or_insert(0) += count*c;
            }
        }
    }

    let (p1_win, p2_win) : (Vec<_>, Vec<_>) =
        map.into_iter().partition(|&(k,_)| k.score.0 >= 21);
    let p1_count = p1_win.into_iter().map(|(_,v)| v).sum();
    let p2_count = p2_win.into_iter().map(|(_,v)| v).sum();
    max(p1_count, p2_count)
}
