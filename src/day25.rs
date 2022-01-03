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

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
}

#[derive(PartialEq,Eq)]
enum Move {
    Down,
    Right,
}

#[derive(PartialEq,Eq)]
struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<Option<Move>>>
}

fn parse(s: &str) -> Map {
    let mut result = Vec::new();
    for line in s.split_whitespace() {
        let mut line_vec = Vec::new();
        for c in line.chars() {
            line_vec.push(match c {
                '>' => Some(Move::Right),
                'v' => Some(Move::Down),
                _ => None,
            });
        }
        result.push(line_vec);
    }
    let width = result[0].len();
    let height = result.len();
    Map { map : result, width, height }
}

fn run_right_step(m: &Map) -> Map {
    let mut result = Vec::new();
    for _ in 0..m.height {
        let mut line = Vec::new();
        for _ in 0..m.width {
            line.push(None);
        }
        result.push(line);
    }

    for (y, line) in m.map.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if let Some(point) = point {
                match point {
                    Move::Right => if m.map[y][(x+1) % m.width].is_none() {
                        result[y][(x+1) % m.width] = Some(Move::Right)
                    } else {
                        result[y][x] = Some(Move::Right)
                    },
                    Move::Down => result[y][x] = Some(Move::Down),
                }
            }
        }
    }

    Map {
        width: m.width,
        height: m.height,
        map: result
    }
}

fn run_down_step(m: &Map) -> Map {
    let mut result = Vec::new();
    for _ in 0..m.height {
        let mut line = Vec::new();
        for _ in 0..m.width {
            line.push(None);
        }
        result.push(line);
    }

    for (y, line) in m.map.iter().enumerate() {
        for (x, point) in line.iter().enumerate() {
            if let Some(point) = point {
                match point {
                    Move::Right => result[y][x] = Some(Move::Right),
                    Move::Down => if m.map[(y+1) % m.height][x].is_none() {
                        result[(y+1) % m.height][x] = Some(Move::Down)
                    } else {
                        result[y][x] = Some(Move::Down)
                    },
                }
            }
        }
    }

    Map {
        width: m.width,
        height: m.height,
        map: result
    }
}

fn run_step(m: &Map) -> Map {
    run_down_step(&run_right_step(m))
}

fn first_part(s: &str) -> usize {
    let mut map = parse(s);
    let mut count = 0;

    loop {
        let new_map = run_step(&map);
        count += 1;

        if new_map == map {
            break
        }
        map = new_map
    }

    count
}
