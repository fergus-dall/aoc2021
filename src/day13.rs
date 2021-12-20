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
use std::collections::HashSet;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    second_part(&str);
}

#[derive(Copy, Clone)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

struct Problem {
    points: HashSet<(usize, usize)>,
    folds: Vec<Fold>,
}

fn parse(s: &str) -> Problem {
    let (points_str, folds_str) = s.split_once("\n\n").unwrap();

    let mut points = HashSet::new();
    for line in points_str.split_whitespace() {
        let (x,y) = line.split_once(',').unwrap();
        points.insert((x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut folds = Vec::new();
    for line in folds_str.split_terminator('\n') {
        let xy = line.chars().nth(11).unwrap();
        let idx = line[13..].parse().unwrap();

        folds.push(match xy {
            'y' => Fold::Horizontal(idx),
            'x' => Fold::Vertical(idx),
            _ => unreachable!(),
        });
    }

    Problem { points, folds }
}

fn do_fold(points: HashSet<(usize, usize)>, fold: Fold) -> HashSet<(usize, usize)> {
    let fold : Box<dyn Fn((usize,usize)) -> (usize,usize)> =
        match fold {
            Fold::Horizontal(idx) => Box::new(move |(x,y)| {
                if y < idx {
                    (x,y)
                } else {
                    (x, idx - (y-idx))
                }
            }),
            Fold::Vertical(idx) => Box::new(move |(x,y)| {
                if x < idx {
                    (x,y)
                } else {
                    (idx - (x-idx), y)
                }
            }),
        };

    let mut result = HashSet::new();
    for point in points {
        result.insert(fold(point));
    }

    result
}

fn first_part(s: &str) -> usize {
    let p = parse(s);
    do_fold(p.points, p.folds[0]).len()
}

fn second_part(s: &str) {
    let p = parse(s);

    let mut points = p.points;
    for fold in p.folds {
        points = do_fold(points, fold);
    }

    for y in 0..7 {
        for x in 0..40 {
            if points.contains(&(x,y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}
