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
use std::ops::Range;
use std::cmp::{min, max};

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Cuboid {
    x: Range<isize>,
    y: Range<isize>,
    z: Range<isize>,
}

struct Op {
    on: bool,
    cuboid: Cuboid,
}

fn range_low(a: &Range<isize>, b: &Range<isize>) -> Range<isize> {
    (a.start)..(min(a.end, b.start))
}

fn range_int(a: &Range<isize>, b: &Range<isize>) -> Range<isize> {
    (max(a.start, b.start))..(min(a.end, b.end))
}

fn range_high(a: &Range<isize>, b: &Range<isize>) -> Range<isize> {
    (max(a.start, b.end))..(a.end)
}

// Return an iterator of non-overlapping cuboids that cover the points
// in the base cuboid but not the other cuboid.
fn cube_diff(base: &Cuboid, other: &Cuboid) -> impl Iterator<Item=Cuboid> {
    IntoIterator::into_iter([
        Cuboid {
            x: range_low(&base.x, &other.x),
            y: base.y.clone(),
            z: base.z.clone(),
        },
        Cuboid {
            x: range_high(&base.x, &other.x),
            y: base.y.clone(),
            z: base.z.clone(),
        },
        Cuboid {
            x: range_int(&base.x, &other.x),
            y: range_low(&base.y, &other.y),
            z: base.z.clone(),
        },
        Cuboid {
            x: range_int(&base.x, &other.x),
            y: range_high(&base.y, &other.y),
            z: base.z.clone(),
        },
        Cuboid {
            x: range_int(&base.x, &other.x),
            y: range_int(&base.y, &other.y),
            z: range_low(&base.z, &other.z),
        },
        Cuboid {
            x: range_int(&base.x, &other.x),
            y: range_int(&base.y, &other.y),
            z: range_high(&base.z, &other.z),
        },
    ]).filter(|cuboid| {
        !cuboid.x.is_empty() && !cuboid.y.is_empty() && !cuboid.z.is_empty()
    })
}

fn clip_cube(base: &Cuboid, clip: &Cuboid) -> Option<Cuboid> {
    let result = Cuboid {
        x: range_int(&base.x, &clip.x),
        y: range_int(&base.y, &clip.y),
        z: range_int(&base.z, &clip.z),
    };

    if result.x.is_empty() || result.y.is_empty() || result.z.is_empty() {
        None
    } else {
        Some(result)
    }
}

fn diff_cubes<'a, I>(cubes: I, diff: &'a Cuboid)
                  -> impl Iterator<Item=Cuboid> + 'a
    where I: Iterator<Item=Cuboid> + 'a {

    cubes.map(move |c| cube_diff(&c, diff)).flatten()
}

fn count_cubes<I: Iterator<Item=Cuboid>>(it: I) -> usize {
    it.map(|c| {
        (c.x.end - c.x.start) as usize *
        (c.y.end - c.y.start) as usize *
        (c.z.end - c.z.start) as usize
    }).sum()
}

fn run_ops(ops: Vec<Op>, clip: bool) -> Vec<Cuboid> {
    let clip_cuboid = Cuboid {
        x: -50..51,
        y: -50..51,
        z: -50..51,
    };

    let mut result = Vec::new();
    for op in ops {
        let diff = if clip {
            match clip_cube(&op.cuboid, &clip_cuboid) {
                Some(x) => x,
                None => continue,
            }
        } else {
            op.cuboid
        };

        result = diff_cubes(result.into_iter(), &diff).collect();

        if op.on {
            result.push(diff)
        };
    }
    result
}

fn parse_ops(s: &str) -> Vec<Op> {
    let mut result = Vec::new();

    for line in s.split_terminator('\n') {
        let (on_off, cuboid) = line.split_once(' ').unwrap();
        let on_off = on_off == "on";
        
        let mut it = cuboid.split(',');
        let (_, x_range) = it.next().unwrap().split_once('=').unwrap();
        let (start, end) = x_range.split_once("..").unwrap();
        let x_range = start.parse::<isize>().unwrap()..(end.parse::<isize>().unwrap() + 1);

        let (_, y_range) = it.next().unwrap().split_once('=').unwrap();
        let (start, end) = y_range.split_once("..").unwrap();
        let y_range = start.parse::<isize>().unwrap()..(end.parse::<isize>().unwrap() + 1);

        let (_, z_range) = it.next().unwrap().split_once('=').unwrap();
        let (start, end) = z_range.split_once("..").unwrap();
        let z_range = start.parse::<isize>().unwrap()..(end.parse::<isize>().unwrap() + 1);

        result.push(Op {
            on: on_off,
            cuboid: Cuboid {
                x: x_range,
                y: y_range,
                z: z_range,
            },
        })
    }

    result
}

fn first_part(s: &str) -> usize {
    let ops = parse_ops(s);
    count_cubes(run_ops(ops, true).into_iter())
}

fn second_part(s: &str) -> usize {
    let ops = parse_ops(s);
    count_cubes(run_ops(ops, false).into_iter())
}
