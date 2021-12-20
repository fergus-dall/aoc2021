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
use std::collections::{HashMap, HashSet};

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Region {
    x: Range<isize>,
    y: Range<isize>,
}

fn parse(s: &str) -> Region {
    let s = s.trim().strip_prefix("target area: x=").unwrap();
    let (x_str, y_str) = s.split_once(", y=").unwrap();
    let (x1, x2) = x_str.split_once("..").unwrap();
    let (y1, y2) = y_str.split_once("..").unwrap();

    Region {
        x: x1.parse().unwrap()..x2.parse::<isize>().unwrap()+1,
        y: y1.parse().unwrap()..y2.parse::<isize>().unwrap()+1,
    }
}

fn first_part(s: &str) -> isize {
    let r = parse(s);

    let mut best_y = 0;
    for y_init in 0..(-r.y.start) {
        let mut sum = 0;
        for y_end in y_init..(-r.y.start) {
            sum += y_end;
            if r.y.contains(&-sum) {
                best_y = y_init
            }
        }
    }

    best_y*(best_y+1)/2
}

fn second_part(s: &str) -> isize {
    let r = parse(s);

    let mut count = 0;
    for x_init in 1..r.x.end {
        for y_init in r.y.start..-r.y.start {
            let mut x_pos = 0;
            let mut y_pos = 0;
            let mut x_vel = x_init;
            let mut y_vel = y_init;

            while (x_vel > 0 || r.x.contains(&x_pos)) && y_pos > r.y.start {
                x_pos += x_vel;
                y_pos += y_vel;
                x_vel -= x_vel.signum();
                y_vel -= 1;

                if r.x.contains(&x_pos) && r.y.contains(&y_pos) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}
