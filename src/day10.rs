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

enum ParseResult {
    Corrupt(char),
    Incomplete(String),
    Complete,
}

fn parse(line: &str) -> ParseResult {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ => match stack.pop() {
                Some(d) => if c != d {
                    return ParseResult::Corrupt(c)
                },
                None => return ParseResult::Corrupt(c)
            },
        }
    }

    if stack.len() == 0 {
        ParseResult::Complete
    } else {
        ParseResult::Incomplete(stack.iter().rev().collect())
    }
}

fn first_part(s: &str) -> usize {
    let mut result = 0;
    for l in s.split_whitespace() {
        match parse(l) {
            ParseResult::Corrupt(c) => match c {
                ')' => result += 3,
                ']' => result += 57,
                '}' => result += 1197,
                '>' => result += 25137,
                _ => unreachable!(),
            },
            _ => {},
        }
    }
    result
}

fn second_part(s: &str) -> usize {
    let mut scores = Vec::new();
    for l in s.split_whitespace() {
        let mut score = 0;
        match parse(l) {
            ParseResult::Incomplete(stack) => {
                for c in stack.chars() {
                    score *= 5;
                    match c {
                        ')' => score += 1,
                        ']' => score += 2,
                        '}' => score += 3,
                        '>' => score += 4,
                        _ => unreachable!(),
                    }
                }
                scores.push(score);
            },
            _ => {},
        }
    }
    scores.sort_unstable();
    scores[scores.len()/2]
}
