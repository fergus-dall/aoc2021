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
    let mut old : isize = -1;
    let mut result : isize = -1;
    for part in s.split("\n") {
        let curr = match FromStr::from_str(part) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if curr > old {
            result += 1
        };
        old = curr;
    }
    result
}

fn second_part(s: &str) -> isize {
    let mut nums = vec!();

    for part in s.split("\n") {
        let i : isize = match FromStr::from_str(part) {
            Ok(i) => i,
            Err(_) => continue,
        };

        nums.push(i);
    }

    let mut old_sum = -1;
    let mut result = -1;
    for i in 0..(nums.len()-2) {
        let new_sum = nums[i] + nums[i+1] + nums[i+2];

        if new_sum > old_sum {
            result += 1
        };
        old_sum = new_sum;
    }

    result
}
