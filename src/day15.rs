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
use std::cmp::Reverse;
use std::collections::{HashSet, HashMap, BinaryHeap};

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

fn parse(s: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for line in s.split_whitespace() {
        let mut new_line = Vec::new();
        for c in line.chars() {
            new_line.push((c as u32 - '0' as u32) as u8);
        }
        result.push(new_line);
    }
    result
}

fn tile(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let height = map.len();
    let width = map[0].len();

    let mut result = Vec::new();
    for y in 0..height*5 {
        let mut line = Vec::new();
        for x in 0..width*5 {
            let inc = (y/height + x/width) as u8;

            line.push(((map[y % height][x % width] + inc - 1) % 9) + 1);
        }
        result.push(line);
    }
    result
}

fn get_neighbors(x: usize, y: usize, width: usize, height: usize)
                 -> impl Iterator<Item=(usize,usize)> {
    IntoIterator::into_iter([(-1,0), (1,0), (0,1), (0,-1)])
        .filter(move |&(dx,dy)| {
            !(x == 0 && dx < 0 ||
              x == width-1 && dx > 0 ||
              y == 0 && dy < 0 ||
              y == height-1 && dy > 0 ||
              dx == 0 && dy == 0)
        })
        .map(move |(dx,dy)| ((x as isize + dx) as usize,
                             (y as isize + dy) as usize))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct FrointierPoint {
    est_cost: usize,
    actual_cost: usize,
    x: usize,
    y: usize,
}

fn a_star(map: &Vec<Vec<u8>>) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut visited = HashSet::with_capacity(height*width);
    let mut frontier = BinaryHeap::new();
    let mut frontier_map = HashMap::new();

    let new_point = |x,y,cost| {
        Reverse(FrointierPoint{
            x: x,
            y: y,
            actual_cost: cost,
            est_cost: cost + (height-y) + (width-x) - 2,
        })
    };

    frontier.push(new_point(0,0,0));

    loop {
        let point = frontier.pop().unwrap().0;

        frontier_map.remove(&(point.x, point.y));

        for (nx,ny) in get_neighbors(point.x,point.y,width,height) {
            if visited.contains(&(nx,ny)) {
                continue;
            }

            let new_actual_cost = point.actual_cost + map[ny][nx] as usize;

            if nx == width-1 && ny == height-1 {
                return new_actual_cost;
            }

            visited.insert((point.x, point.y));

            // It's possible that we will find a better path to a node
            // before expanding it, in which case we need to add it to
            // the priority queue a second time with the new improved
            // estimated cost.
            if !frontier_map.contains_key(&(nx,ny)) ||
                frontier_map[&(nx,ny)] > new_actual_cost {

                frontier_map.insert((nx,ny), new_actual_cost);
                frontier.push(new_point(nx,ny,new_actual_cost));
            }
        }
    }
}

fn first_part(s: &str) -> usize {
    let map = parse(s);
    a_star(&map)
}

fn second_part(s: &str) -> usize {
    let map = parse(s);
    let map = tile(map);
    a_star(&map)
}
