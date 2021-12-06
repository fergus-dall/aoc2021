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
    println!("{}", second_part(&str));
}

fn parse(s: &str) -> [usize; 9] {
    let mut r = [0; 9];
    for n in s.split(',').map(|n| -> usize {
        n.trim().parse().expect("Failed to parse")
    }) {
        r[n] += 1;
    }
    r
}

fn iterate(v: [usize; 9]) -> [usize; 9] {
    [v[1], v[2], v[3], v[4], v[5], v[6], v[7]+v[0], v[8], v[0]]
}

fn first_part(s: &str) -> usize {
    let mut s = parse(s);
    for _ in 0..80 {
        s = iterate(s);
    }
    s.iter().fold(0, |x,y| x+y)
}

fn second_part(s: &str) -> usize {
    let mut s = parse(s);
    for _ in 0..256 {
        s = iterate(s);
    }
    s.iter().fold(0, |x,y| x+y)
}
