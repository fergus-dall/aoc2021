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
use std::hash::Hash;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Problem {
    pairs: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), ((char, char), (char, char))>,
    first: char,
    last: char,
}

fn add<K>(map: &mut HashMap<K, usize>, key: K, val: usize)
    where K: Eq + Copy + Hash {
    if map.contains_key(&key) {
        map.insert(key, map[&key] + val);
    } else {
        map.insert(key, val);
    }
}

fn parse(s: &str) -> Problem {
    let (template_str, rules_str) = s.split_once("\n\n").unwrap();

    let mut template = HashMap::new();
    for pair in template_str.chars().zip(template_str.chars().skip(1)) {
        add(&mut template, pair, 1)
    }

    let mut rules = HashMap::new();
    for rule in rules_str.split_terminator('\n') {
        let (input, out) = rule.split_once(" -> ").unwrap();

        let input1 = input.chars().nth(0).unwrap();
        let input2 = input.chars().nth(1).unwrap();
        let input = (input1, input2);

        let out = out.chars().nth(0).unwrap();
        let out1 = (input1, out);
        let out2 = (out, input2);
        let out = (out1, out2);

        rules.insert(input, out);
    }

    Problem { pairs: template,
              rules,
              first: template_str.chars().next().unwrap(),
              last: template_str.chars().last().unwrap(),
    }
}

fn step(mut problem: Problem) -> Problem {
    let mut new_pairs = HashMap::new();

    for (pair, count) in problem.pairs.into_iter() {
        if problem.rules.contains_key(&pair) {
            let (first, second) = problem.rules[&pair];
            add(&mut new_pairs, first, count);
            add(&mut new_pairs, second, count);
        } else {
            add(&mut new_pairs, pair, count);
        }
    }

    problem.pairs = new_pairs;
    problem
}

fn count(problem: Problem) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    add(&mut result, problem.first, 1);
    add(&mut result, problem.last, 1);

    for (pair, count) in problem.pairs.into_iter() {
        add(&mut result, pair.0, count);
        add(&mut result, pair.1, count);
    }

    result.into_iter().map(|(pair, count)| (pair, count/2)).collect()
}

fn run_steps(mut problem: Problem, steps: usize) -> usize {
    for _ in 0..steps {
        problem = step(problem);
    }
    let count = count(problem);

    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for (_, count) in count {
        if count < min {
            min = count;
        }
        if count > max {
            max = count;
        }
    }

    max - min
}

fn first_part(s: &str) -> usize {
    let problem = parse(s);
    run_steps(problem, 10)
}

fn second_part(s: &str) -> usize {
    let problem = parse(s);
    run_steps(problem, 40)
}
