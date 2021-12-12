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
use std::collections::HashMap;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Graph<'a> {
    big_caves: HashMap<&'a str, Vec<&'a str>>,
    small_caves: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn parse(s: &'a str) -> Result<Graph<'a>, &'static str> {
        let mut result = Graph {
            big_caves: HashMap::new(),
            small_caves: HashMap::new()
        };

        for line in s.split_whitespace() {
            let (from, to) = line.split_once('-')
                .ok_or("Couldn't find '-'")?;

            if from.len() == 0 {
                return Err("From is empty")
            }
            if to.len() == 0 {
                return Err("To is empty")
            }

            result.insert(from, to);
            result.insert(to, from);
        }

        Ok(result)
    }

    fn is_big(s: &str) -> bool {
        s.chars().next().map(|x| char::is_ascii_uppercase(&x)).unwrap()
    }

    fn insert(&mut self, from: &'a str, to: &'a str) {
        let map = if Graph::is_big(from) {
            &mut self.big_caves
        } else {
            &mut self.small_caves
        };

        let vec = match map.get_mut(from) {
            Some(v) => v,
            None => {
                map.insert(from, Vec::new());
                map.get_mut(from).unwrap()
            },
        };
        vec.push(to);
    }
}

fn search<'a>(graph: &Graph<'a>,
              candidate_path: &mut Vec<&'a str>,
              allow_dup: bool) -> usize {
    let curr_node = candidate_path[candidate_path.len()-1];

    let links = if Graph::is_big(curr_node) {
        graph.big_caves.get(curr_node).unwrap()
    } else {
        graph.small_caves.get(curr_node).unwrap()
    };

    let mut result = 0;
    for &link in links {
        let mut allow_dup = allow_dup;

        if !Graph::is_big(link) &&
            candidate_path.iter().any(|&x| x == link)
        {
            if allow_dup {
                allow_dup = false;
            } else {
                continue
            }
        }

        if link == "start" {
            continue
        }

        if link == "end" {
            result += 1;
        } else {
            candidate_path.push(link);
            result += search(graph, candidate_path, allow_dup);
            candidate_path.pop();
        }
    }
    result
}

fn first_part(s: &str) -> usize {
    let graph = Graph::parse(s).unwrap();

    let mut candidate_path = Vec::new();
    candidate_path.push("start");
    search(&graph, &mut candidate_path, false)
}

fn second_part(s: &str) -> usize {
    let graph = Graph::parse(s).unwrap();

    let mut candidate_path = Vec::new();
    candidate_path.push("start");
    search(&graph, &mut candidate_path, true)
}
