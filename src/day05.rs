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
use std::cmp::{Ordering,Ord};

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

fn parse_point(s: &str) -> Result<(usize, usize), &'static str> {
    let idx = s.find(',').ok_or("Couldn't find ','")?;
    let (before, after) = s.split_at(idx);
    Ok((before.parse().or(Err("Not a number"))?,
        after.chars().skip(1)
        .collect::<String>().parse().or(Err("Not a  number"))?))
}

fn parse(s: &str) -> Result<Vec<((usize, usize), (usize, usize))>, &'static str> {
    s.split_terminator('\n').map(|line| {
        let idx = line.find(" -> ").ok_or("Couldn't find \"->\"")?;
        let (before, after) = line.split_at(idx);

        Ok((parse_point(before)?,
            parse_point(&after.chars().skip(4).collect::<String>())?))   
    }).collect()
}

fn ord_to_sign(ord: Ordering) -> isize {
    match ord {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    }
}

fn inc(pos: usize, dir: isize) -> usize {
    ((pos as isize) + dir) as usize
}

fn mk_map(v: &Vec<((usize, usize), (usize, usize))>, diag: bool) -> Vec<Vec<usize>> {
    let mut r: Vec<Vec<usize>> = Vec::new();

    for line in v {
        let mut x = line.0.0;
        let mut y = line.0.1;
        let x_dir = ord_to_sign(x.cmp(&line.1.0));
        let y_dir = ord_to_sign(y.cmp(&line.1.1));

        if x_dir != 0 && y_dir != 0 && !diag {
            continue
        }

        while x != inc(line.1.0, x_dir) || y != inc(line.1.1, y_dir) {
            while r.len() <= x {
                r.push(Vec::new())
            }
            while r[x].len() <= y {
                r[x].push(0)
            }
            
            r[x][y] += 1;
            x = inc(x, x_dir);
            y = inc(y, y_dir);
        }
    }
    r
}

fn first_part(s: &str) -> usize {
    let lines = parse(s).expect("Failed to parse file");

    mk_map(&lines, false).iter().flatten().filter(|i| **i >= 2).count()
}

fn second_part(s: &str) -> usize {
    let lines = parse(s).expect("Failed to parse file");

    mk_map(&lines, true).iter().flatten().filter(|i| **i >= 2).count()
}
