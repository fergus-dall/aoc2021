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
use std::ops::Range;
use std::collections::HashSet;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Problem {
    points: HashSet<(isize, isize)>,
    trans: Vec<bool>,
}

fn parse(s: &str) -> Problem {
    let (alg, img) = s.split_once("\n\n").unwrap();

    let mut points = HashSet::new();
    for (y,line) in img.split_whitespace().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == '#' {
                points.insert((x as isize, y as isize));
            }
        }
    }

    let trans = alg.chars().map(|c| c == '#').collect::<Vec<bool>>();

    Problem { points, trans }
}

fn two_steps(region: [[bool; 5]; 5], trans: &Vec<bool>) -> bool {
    let mut middle_step = 0;
    for i in 1..4 {
        for j in 1..4 {
            let mut code = 0;
            for dy in -1..2 {
                for dx in -1..2 {
                    code <<= 1;
                    if region[(i+dy) as usize][(j+dx) as usize] {
                        code += 1;
                    }
                }
            }

            middle_step <<= 1;
            if trans[code] {
                middle_step += 1;
            }
        }
    }

    trans[middle_step]
}

fn generate_patches(points: &HashSet<(isize, isize)>, range: Range<isize>) -> Vec<([[bool; 5]; 5], isize, isize)> {

    let mut result = Vec::new();
    for x in range.clone() {
        for y in range.clone() {
            let mut patch = [[false; 5]; 5];
            for dx in 0..5 {
                for dy in 0..5 {
                    if points.contains(&(x+dx, y+dy)) {
                        patch[dy as usize][dx as usize] = true;
                    }
                }
            }
            result.push((patch, x, y));
        }
    }
    result
}

fn first_part(s: &str) -> usize {
    let problem = parse(s);

    let mut result = 0;
    for (patch,_,_) in generate_patches(&problem.points, -4..100) {
        if two_steps(patch, &problem.trans) {
            result += 1;
        }
    }

    result
}

fn second_part(s: &str) -> usize {
    let problem = parse(s);

    let mut points = problem.points;
    for _ in 0..25 {
        let mut new_points = HashSet::new();
        for (patch,x,y) in generate_patches(&points, -200..300) {
            if two_steps(patch, &problem.trans) {
                new_points.insert((x,y));
            }
        }
        points = new_points;
    }

    points.len()
}
