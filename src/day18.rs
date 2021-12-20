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

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

#[derive(Debug, Clone)]
enum Val {
    Num(u8),
    Pair(Box<(Val, Val)>),
}

fn parse(s: &str) -> Vec<Val> {
    let mut result = Vec::new();
    for line in s.split_whitespace() {
        let mut stack = Vec::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                stack.push(Val::Num((c as u32 - '0' as u32) as u8));
            } else if c == ']' {
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                stack.push(Val::Pair(Box::new((v1,v2))));
            }
        }
        result.push(stack.pop().unwrap());
    }
    result
}

fn search_explode(pos: &mut Val, left: Option<&mut Val>, right: Option<&mut Val>, depth: usize) -> bool {
    if depth == 4 {
        match pos {
            Val::Num(_) => return false,
            Val::Pair(b) => {
                if let Some(mut left) = left {
                    while let Val::Pair(b) = left {
                        left = &mut b.1
                    }
                    if let Val::Num(left) = left {
                        if let Val::Num(new) = b.0 {
                            *left += new;
                        }
                    }
                };

                if let Some(mut right) = right {
                    while let Val::Pair(b) = right {
                        right = &mut b.0
                    }
                    if let Val::Num(right) = right {
                        if let Val::Num(new) = b.1 {
                            *right += new;
                        }
                    }
                };

                *pos = Val::Num(0);
                return true;
            },
        }
    };

    match pos {
        Val::Num(_) => false,
        Val::Pair(b) => {
            search_explode(&mut b.0, left, Some(&mut b.1), depth+1) ||
            search_explode(&mut b.1, Some(&mut b.0), right, depth+1)
        }
    }
}

fn split(pos: &mut Val) -> bool {
    match pos {
        Val::Pair(b) => split(&mut b.0) || split(&mut b.1),
        &mut Val::Num(x) => {
            if x >= 10 {
                *pos = Val::Pair(Box::new((
                    Val::Num(x/2),
                    Val::Num(x/2 + x%2),
                    )));

                true
            } else {
                false
            }
        },
    }
}

fn add(v1: Val, v2: Val) -> Val {
    let mut result = Val::Pair(Box::new((v1, v2)));

    loop {
        if search_explode(&mut result, None, None, 0) || split(&mut result) {
        } else {
            break
        }
    }

    result
}

fn magnitude(v: Val) -> usize {
    match v {
        Val::Num(x) => x as usize,
        Val::Pair(b) => 3*magnitude(b.0) + 2*magnitude(b.1),
    }
}

fn first_part(s: &str) -> usize {
    magnitude(parse(s).into_iter().reduce(add).unwrap())
}

fn second_part(s: &str) -> usize {
    parse(s).into_iter().permutations(2)
        .map(|mut v| {
            add(v.pop().unwrap(),
                v.pop().unwrap())
        })
        .map(magnitude).max().unwrap()
}
