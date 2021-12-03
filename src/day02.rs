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

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

fn first_part(s: &str) -> isize {
    let mut horz = 0;
    let mut depth = 0;

    for part in s.split("\n") {
        let idx = match part.find(" ") {
            Some(i) => i,
            None => continue,
        };

        let n = match isize::from_str(part.split_at(idx+1).1) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if part.starts_with("up") {
            depth -= n;
        } else if part.starts_with("down") {
            depth += n;
        } else if part.starts_with("forward") {
            horz += n;
        } else {
            continue
        }
    };

    horz*depth
}

fn second_part(s: &str) -> isize {
    let mut horz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for part in s.split("\n") {
        let idx = match part.find(" ") {
            Some(i) => i,
            None => continue,
        };

        let n = match isize::from_str(part.split_at(idx+1).1) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if part.starts_with("up") {
            aim -= n;
        } else if part.starts_with("down") {
            aim += n;
        } else if part.starts_with("forward") {
            horz += n;
            depth += aim * n;
        } else {
            continue
        }
    };

    horz*depth
}
