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

fn parse(s: &str) -> Vec<usize> {
    let mut result = Vec::new();
    for n in s.split(',') {
        let n = n.trim().parse().expect("Couldn't parse value");

        while result.len() <= n {
            result.push(0);
        }

        result[n] += 1;
    }

    result
}

fn find_best<T: Fn(usize) -> usize>(v: &Vec<usize>, f: T) -> usize {
    let mut best_cost = usize::MAX;
    for guess in 0..v.len() {
        let cost = v.iter().enumerate()
            .map(|(i,x)| {
                let dist = (guess as isize - i as isize).abs() as usize;
                f(dist) * x
            })
            .fold(0, |x,y| x+y);

        if cost < best_cost {
            best_cost = cost;
        }
    }

    best_cost
}

fn first_part(s: &str) -> usize {
    let v = parse(s);
    find_best(&v, |dist| dist)
}

fn second_part(s: &str) -> usize {
    let v = parse(s);
    find_best(&v, |dist| dist*(dist+1)/2)
}
