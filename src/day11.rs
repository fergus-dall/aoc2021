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
use itertools::Itertools;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Grid {
    map: [[u32; 10]; 10],
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        for line in s.split_whitespace() {
            let mut l = Vec::new();
            for c in line.chars() {
                l.push(c as u32 - '0' as u32);
            }
            map.push(l.try_into().or(
                Err("Line wasn't 10 characters long!"))?);
        }
        Ok(Grid { map: map.try_into().or(
        Err("Map wasn't 10 lines long!"))? })
    }
}

fn get_neighbors(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
    (-1..2isize).cartesian_product(-1..2isize)
        .filter(move |&(dx,dy)| {
            !(x == 0 && dx < 0 ||
              x == 9 && dx > 0 ||
              y == 0 && dy < 0 ||
              y == 9 && dy > 0 ||
              dx == 0 && dy == 0)
        })
        .map(move |(dx,dy)| ((x as isize + dx) as usize,
                             (y as isize + dy) as usize))
}

impl Grid {
    fn step(&mut self) -> [[bool; 10]; 10] {
        let mut result = [[false; 10]; 10];

        for x in 0..10 {
            for y in 0..10 {
                self.map[x][y] += 1
            }
        }

        let mut done = false;
        while !done {
            done = true;

            for x in 0..10 {
                for y in 0..10 {
                    if self.map[x][y] > 9 && !result[x][y] {
                        done = false;
                        result[x][y] = true;
                        for (nx,ny) in get_neighbors(x,y) {
                            self.map[nx][ny] += 1;
                        }
                    }
                }
            }
        }

        for x in 0..10 {
            for y in 0..10 {
                if result[x][y]{
                    self.map[x][y] = 0;
                }
            }
        }

        result
    }
}

fn first_part(s: &str) -> usize {
    let mut grid: Grid = s.parse().expect("Failed to parse problem");
    let mut result = 0;
    for _ in 0..100 {
        let flashes = grid.step();
        for i in flashes.iter().flatten().cloned() {
            if i {
                result += 1;
            }
        }
    }
    result
}

fn second_part(s: &str) -> usize {
    let mut grid: Grid = s.parse().expect("Failed to parse problem");

    let mut i = 0;
    loop {
        i += 1;
        let flashes = grid.step().iter().flatten().filter(|&&x| x).count();
        if flashes == 100 {
            return i
        }
    }
}
