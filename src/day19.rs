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
use std::convert::TryInto;
use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;
use itertools::Itertools;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct ThreeVec {
    coord: [isize; 3],
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Orient {
    o: [(usize, bool); 3],
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct ProbePos {
    pos: ThreeVec,
    orient: Orient,
}

struct Readings {
    probes: Vec<Vec<ThreeVec>>,
}

fn parse(s: &str) -> Readings {
    let mut result = Vec::new();
    for probe in s.split("\n\n") {
        let mut probe_scan = Vec::new();
        for line in probe.split_terminator('\n').skip(1) {
            probe_scan.push(
                ThreeVec {
                    coord :
                    line.split(',')
                        .map(|s| {
                            s.parse().unwrap()
                        })
                        .collect::<Vec<isize>>().try_into()
                        .unwrap()
                })
        }
        result.push(probe_scan);
    }

    Readings { probes: result }
}

fn rotate(pos: ThreeVec, or: Orient) -> ThreeVec {
    ThreeVec {
        coord: [
            pos.coord[or.o[0].0] * if or.o[0].1 { -1 } else { 1 },
            pos.coord[or.o[1].0] * if or.o[1].1 { -1 } else { 1 },
            pos.coord[or.o[2].0] * if or.o[2].1 { -1 } else { 1 },
            ]
    }
}

fn diff(p: ThreeVec, q: ThreeVec) -> ThreeVec {
    ThreeVec { coord :
                [q.coord[0] - p.coord[0],
                 q.coord[1] - p.coord[1],
                 q.coord[2] - p.coord[2]] }
}

fn add(p: ThreeVec, q: ThreeVec) -> ThreeVec {
    ThreeVec { coord :
                [q.coord[0] + p.coord[0],
                 q.coord[1] + p.coord[1],
                 q.coord[2] + p.coord[2]] }
}

fn maybe_eq(p: &ThreeVec, q: &ThreeVec) -> Option<Orient> {
    'outer: for i in p.coord {
        for j in q.coord {
            if i.abs() == j.abs() {
                continue 'outer;
            }
        }
        return None;
    }

    let p = p.coord;
    let mut q = q.coord;
    let mut o = [(0,false),(1,false),(2,false)];
    while p[0].abs() != q[0].abs() {
        q = [q[1], q[2], q[0]];
        o = [o[1], o[2], o[0]];
    }

    let swap = p[1].abs() != q[1].abs();
    if swap {
        q = [q[0], q[2], q[1]];
        o = [o[0], o[2], o[1]];
    }
    let mut negations = 0;

    for i in 0..3 {
        if p[i] != q[i] {
            negations += 1;
            o[i].1 = true;
        }
    }

    if swap && negations % 2 != 0 {
        Some(Orient { o })
    } else if !swap && negations % 2 == 0 {
        Some(Orient { o })
    } else {
        None
    }
}

fn position_probe<'a>(base: &'a Vec<ThreeVec>, other: &'a Vec<ThreeVec>) -> Option<ProbePos> {
    let get_diffs = |v : &'a Vec<ThreeVec>| {
        v.iter().permutations(2)
            .map(|v| {
                (v[0], v[1], diff(*v[0], *v[1]))
            })
    };

    let it = get_diffs(base).cartesian_product(get_diffs(other))
        .map(|(i,j)| {
            let ret = maybe_eq(&i.2,&j.2);
            (i,j, ret)
        }).filter(|(_,_,x)| x.is_some())
        .map(|(i,j,x)| (i,j,x.unwrap()));

    let mut count = HashMap::new();
    for (_,_,pos) in it.clone() {
        *count.entry(pos).or_insert(0) += 1;
    }


    for (o,count) in count.into_iter() {
        if count >= 12 {
            let matchup = it.filter(|&(_,_,or)| or == o).next().unwrap();
            let offset = diff(rotate(*matchup.1.0, o), *matchup.0.0);

            return Some(ProbePos { pos : offset, orient: o });
        }
    }

    None
}

fn compose_probe(p: &ProbePos, q: &ProbePos) -> ProbePos {
    let pos = add(rotate(add(rotate(
        ThreeVec { coord: [0,0,0] }
            , q.orient), q.pos), p.orient), p.pos);
    let orient = Orient { o: [
        (q.orient.o[p.orient.o[0].0].0,
         p.orient.o[0].1 ^ q.orient.o[p.orient.o[0].0].1),
        (q.orient.o[p.orient.o[1].0].0,
         p.orient.o[1].1 ^ q.orient.o[p.orient.o[1].0].1),
        (q.orient.o[p.orient.o[2].0].0,
         p.orient.o[2].1 ^ q.orient.o[p.orient.o[2].0].1),
    ] };

    ProbePos { pos, orient }
}

fn get_probe_map(readings: &Readings) -> Vec<ProbePos> {
    let mut map = HashMap::new();

    for (i,j,pos) in readings.probes.iter()
        .enumerate().permutations(2).map(|v| {
        let pos = position_probe(v[0].1,v[1].1);
        (v[0].0, v[1].0, pos)
        }) {

            if let Some(pos) = pos {
                map.insert((i,j), pos);
            }

        }

    loop {
        let mut done = true;
        let it = map.keys().cloned().collect::<Vec<(usize,usize)>>()
            .into_iter().permutations(2)
            .map(|v| (v[0], v[1])).filter(|((_,i),(j,_))| i == j);
        for ((i,j),(_,k)) in it {

            let comp = compose_probe(&map[&(i,j)], &map[&(j,k)]);

            match map.entry((i,k)) {
                Entry::Occupied(oc) => {
                    if oc.get() != &comp {
                        panic!("BUG: {:?} {:?}", oc.get(), comp)
                    }
                },
                Entry::Vacant(vac) => {
                    vac.insert(comp);
                    done = false;
                },
            }
        }

        if done {
            break
        }
    }

    let mut result = Vec::new();
    for i in 0..readings.probes.len() {
        result.push(map[&(0,i)])
    }
    result
}

fn first_part(s: &str) -> usize {
    let readings = parse(s);

    let map = get_probe_map(&readings);

    let mut beacons = HashSet::new();

    for (idx, beacon) in readings.probes.iter().enumerate() {
        let mapping = map[idx];

        for b in beacon {
            let trans_pos = add(rotate(*b, mapping.orient), mapping.pos);
            beacons.insert(trans_pos);
        }
    }

    beacons.len()
}

fn second_part(s: &str) -> usize {
    let readings = parse(s);

    let map = get_probe_map(&readings);

    let mut max = 0;
    for i in &map {
        for j in &map {
            let new_max = diff(i.pos, j.pos).coord.iter()
                .map(|x| x.abs() as usize).sum();
            if new_max > max {
                max = new_max;
            }
        }
    }
    max
}
