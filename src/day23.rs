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
use std::cmp::{max,min,Reverse};
use std::collections::{HashMap, BinaryHeap};

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

#[derive(Copy,Clone,PartialEq,Eq,Hash,PartialOrd,Ord,Debug)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone,PartialEq,Eq,Hash,PartialOrd,Ord,Debug)]
struct State {
    hallway: [Option<Type>; 11],
    a_room: Vec<Option<Type>>,
    b_room: Vec<Option<Type>>,
    c_room: Vec<Option<Type>>,
    d_room: Vec<Option<Type>>,
}

fn type_to_room(s: &State, t: Type) -> &Vec<Option<Type>> {
    match t {
        Type::Amber => &s.a_room,
        Type::Bronze => &s.b_room,
        Type::Copper => &s.c_room,
        Type::Desert => &s.d_room,
    }
}

fn type_to_room_mut(s: &mut State, t: Type) -> &mut Vec<Option<Type>> {
    match t {
        Type::Amber => &mut s.a_room,
        Type::Bronze => &mut s.b_room,
        Type::Copper => &mut s.c_room,
        Type::Desert => &mut s.d_room,
    }
}

fn type_to_room_idx(t: Type) -> usize {
    match t {
        Type::Amber => 2,
        Type::Bronze => 4,
        Type::Copper => 6,
        Type::Desert => 8,
    }
}

fn moves_between(a: usize, b: usize) -> usize {
    max(a,b) - min(a,b)
}

fn type_to_cost(t: Type) -> usize {
    match t {
        Type::Amber => 1,
        Type::Bronze => 10,
        Type::Copper => 100,
        Type::Desert => 1000,
    }
}

// Can creatures of type a enter the given room?
fn room_open(room: &Vec<Option<Type>>, t: Type) -> bool {
    for i in room {
        if let Some(o) = i {
            if *o != t {
                return false;
            }
        }
    }
    room[0].is_none() // Need an empty tile in the entrace
}

// Is there a clear path to the room at target idx?
fn room_accessible(s: &State, curr: usize, target: usize) -> bool {
    if curr < target {
        for i in (curr+1)..(target+1) {
            if s.hallway[i] != None {
                return false
            }
        }
    } else {
        for i in target..curr {
            if s.hallway[i] != None {
                return false
            }
        }
    }
    true
}

fn get_neighbor_costs(s: &State) -> Vec<(State, usize)> {
    let mut result = Vec::new();

    // Moves from the hallway to a room
    for (idx,h) in s.hallway.iter().cloned().enumerate() {
        let h = if let Some(h) = h {
            h
        } else {
            continue
        };

        if !room_open(type_to_room(s, h), h) {
            continue
        };

        if !room_accessible(s, idx, type_to_room_idx(h)) {
            continue
        }

        let moves = moves_between(type_to_room_idx(h), idx) + 1;

        let mut new_s = s.clone();
        let room = type_to_room_mut(&mut new_s, h);
        room[0] = Some(h);
        new_s.hallway[idx] = None;
        result.push((new_s.clone(), moves * type_to_cost(h)));

        for i in 1..new_s.a_room.len() {
            let room = type_to_room_mut(&mut new_s, h);
            if room[i].is_some() {
                break
            }
            room[i] = room[i-1];
            room[i-1] = None;
            result.push((new_s.clone(), (moves+i) * type_to_cost(h)));
        }
    }

    // Moves from a room to the hallway
    for t in [Type::Amber, Type::Bronze, Type::Copper, Type::Desert] {
        let room = type_to_room(s, t);
        
        let (type_moving, extra_moves) =
            if let Some((idx, &Some(o))) = room.iter().enumerate().find(|(_, o)| o.is_some()) {
                (o, idx+1)
            } else {
                continue
            };

        let src_idx = type_to_room_idx(t);
        for target_idx in 0..11 {
            if target_idx == 2 ||
                target_idx == 4 ||
                target_idx == 6 ||
                target_idx == 8 {
                    continue // no stopping outside of rooms
                };

            if room_accessible(s, src_idx, target_idx) {
                let mut new_s = s.clone();
                let new_room = type_to_room_mut(&mut new_s, t);
                new_room[extra_moves-1] = None;
                new_s.hallway[target_idx] = Some(type_moving);
                let moves = moves_between(src_idx, target_idx) + extra_moves;
                result.push((new_s, moves * type_to_cost(type_moving)));
            }
        }
    }

    result
}

fn is_win_state(s: &State) -> bool {
    s.hallway == [None; 11] &&
        s.a_room.iter().all(|&x| x == Some(Type::Amber)) &&
        s.b_room.iter().all(|&x| x == Some(Type::Bronze)) &&
        s.c_room.iter().all(|&x| x == Some(Type::Copper)) &&
        s.d_room.iter().all(|&x| x == Some(Type::Desert))
}

#[derive(PartialEq,Eq,PartialOrd,Ord)]
struct SearchNode {
    est_cost: usize,
    state: State,
}

fn bfs(s: &State) -> (State, usize) {
    let mut frontier = BinaryHeap::new();
    let mut best_costs = HashMap::new();

    frontier.push(Reverse(SearchNode{
        est_cost: 0,
        state: s.clone(),
    }));
    best_costs.insert(s.clone(), 0);

    while !frontier.is_empty() {
        let s = frontier.pop().unwrap().0.state;
        let current_cost = best_costs[&s];

        if is_win_state(&s) {
            let win_cost = best_costs[&s];
            return (s, win_cost);
        };

        for (new_s, new_cost) in get_neighbor_costs(&s) {
            if best_costs.contains_key(&new_s) {
                if best_costs[&new_s] <= current_cost + new_cost {
                    continue
                }
            };

            best_costs.insert(new_s.clone(), current_cost + new_cost);
            frontier.push(Reverse(SearchNode{
                est_cost: current_cost + new_cost,
                state: new_s,
            }));
        }
    }

    unreachable!()
}

fn parse(s: &str) -> State {
    let mut a_room = Vec::new();
    let mut b_room = Vec::new();
    let mut c_room = Vec::new();
    let mut d_room = Vec::new();

    let mut count = 0;
    for c in s.chars() {
        let tile = match c {
            'A' => Type::Amber,
            'B' => Type::Bronze,
            'C' => Type::Copper,
            'D' => Type::Desert,
            _ => continue,
        };

        let target = match count%4 {
            0 => &mut a_room,
            1 => &mut b_room,
            2 => &mut c_room,
            3 => &mut d_room,
            _ => unreachable!(),
        };

        target.push(Some(tile));
        count += 1;
    }

    State {
        hallway: [None; 11],
        a_room,
        b_room,
        c_room,
        d_room,
    }
}

fn first_part(s: &str) -> usize {
    let state = parse(s);

    let (_, cost) = bfs(&state);

    cost
}

fn second_part(s: &str) -> usize {
    let mut state = parse(s);

    state.a_room = vec![state.a_room[0], Some(Type::Desert),
                        Some(Type::Desert), state.a_room[1]];
    state.b_room = vec![state.b_room[0], Some(Type::Copper),
                        Some(Type::Bronze), state.b_room[1]];
    state.c_room = vec![state.c_room[0], Some(Type::Bronze),
                        Some(Type::Amber), state.c_room[1]];
    state.d_room = vec![state.d_room[0], Some(Type::Amber),
                        Some(Type::Copper), state.d_room[1]];

    let (_, cost) = bfs(&state);

    cost
}
