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
use std::collections::HashSet;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Map {
    map: Vec<usize>,
    width: usize,
    height: usize,
}

fn parse(s: &str) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut map = Vec::new();
    for line in s.split_whitespace() {
        width = line.len();
        height += 1;

        for c in line.chars() {
            map.push(((c as u32) - ('0' as u32)) as usize);
        }
    }

    Map { map, width, height }
}

fn find_low_points(map: &Map) -> impl Iterator<Item=(usize, usize)> + '_ {
    (0..map.map.len())
        .map(move |idx| (idx % map.width, idx / map.width))
        .filter(move |&(x,y)| {
            let val = map.map[y*map.width + x];
            if x != 0 && val >= map.map[y*map.width + x-1] {
                false
            } else if x != (map.width-1) && val >= map.map[y*map.width + x+1] {
                false
            } else if y != 0 && val >= map.map[(y-1)*map.width + x] {
                false
            } else if y != (map.height-1) && val >= map.map[(y+1)*map.width + x] {
                false
            } else {
                true
            }
        })
}

fn find_basin_size(map: &Map, point: (usize, usize)) -> usize {
    let mut scanned = HashSet::new();
    let mut frontier = HashSet::new();
    frontier.insert(point);

    while frontier.len() > 0 {
        scanned = scanned.union(&frontier).cloned().collect();
        let mut new_frontier = HashSet::new();

        for (x,y) in frontier {
            let val = map.map[y*map.width + x];

            if x != 0 {
                let new_val = map.map[y*map.width + x-1];
                if val < new_val && new_val < 9 {
                    new_frontier.insert((x-1, y));
                }
            }

            if x != (map.width-1) {
                let new_val = map.map[y*map.width + x+1];
                if val < new_val && new_val < 9 {
                    new_frontier.insert((x+1, y));
                }
            }

            if y != 0 {
                let new_val = map.map[(y-1)*map.width + x];
                if val < new_val && new_val < 9 {
                    new_frontier.insert((x, y-1));
                }
            }

            if y != (map.height-1) {
                let new_val = map.map[(y+1)*map.width + x];
                if val < new_val && new_val < 9 {
                    new_frontier.insert((x, y+1));
                }
            }
        }

        frontier = new_frontier.difference(&scanned).cloned().collect();
    }

    scanned.len()
}

fn first_part(s: &str) -> usize {
    let map = parse(s);
    find_low_points(&map).fold(0, |sum,(x,y)| {
        sum + map.map[y*map.width + x] + 1
    })
}

fn second_part(s: &str) -> usize {
    let map = parse(s);
    let mut basins = find_low_points(&map)
        .map(|x| find_basin_size(&map, x))
        .collect::<Vec<_>>();
    basins.sort_unstable_by(|x,y| y.cmp(x));
    basins.iter().take(3)
        .fold(1, |x,y| x*y)
}
