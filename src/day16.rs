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
use std::iter::Peekable;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug)]
enum Contents {
    Lit(usize),
    Op(Op, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    contents: Contents,
}

fn bits_to_int<I>(it : &mut Peekable<I>, count: usize) -> usize
    where I: Iterator<Item=(usize,bool)> {

    let mut result = 0;
    for i in 0..count {
        if it.next().unwrap().1 {
            result += 1 << (count-i-1)
        }
    }
    result
}

fn parse_lit_int<I>(it : &mut Peekable<I>) -> usize
    where I: Iterator<Item=(usize,bool)> {

    let mut result = 0;
    let mut cont = true;
    while cont {
        cont = it.next().unwrap().1;

        result = (result << 4) + bits_to_int(it, 4);
    }
    result
}

fn parse_op_packet<I>(it : &mut Peekable<I>) -> Vec<Packet>
    where I: Iterator<Item=(usize,bool)> {

    let mut result = Vec::new();
    if it.next().unwrap().1 {
        let sub_packet_count = bits_to_int(it, 11);

        for _ in 0..sub_packet_count {
            result.push(parse_packet(it));
        }
    } else {
        let sub_packet_len = bits_to_int(it, 15);
        let target_idx = it.peek().unwrap().0 + sub_packet_len;

        loop {
            match it.peek() {
                None => break,
                Some(&(idx,_)) => if idx == target_idx {
                    break
                } else if idx > target_idx {
                    panic!()
                }
            };
            result.push(parse_packet(it));
        }
    }

    result
}

fn parse_packet<I>(it : &mut Peekable<I>) -> Packet
    where I: Iterator<Item=(usize,bool)> {

    let version = bits_to_int(it, 3);
    let type_id = bits_to_int(it, 3);

    let contents = match type_id {
        0 => Contents::Op(Op::Sum, parse_op_packet(it)),
        1 => Contents::Op(Op::Product, parse_op_packet(it)),
        2 => Contents::Op(Op::Min, parse_op_packet(it)),
        3 => Contents::Op(Op::Max, parse_op_packet(it)),
        4 => Contents::Lit(parse_lit_int(it)),
        5 => Contents::Op(Op::Gt, parse_op_packet(it)),
        6 => Contents::Op(Op::Lt, parse_op_packet(it)),
        7 => Contents::Op(Op::Eq, parse_op_packet(it)),
        _ => unreachable!(),
    };

    Packet { version, contents }
}

fn parse(s: &str) -> Packet {
    let mut it = s.trim().chars().map(|c| {
        if ('0'..':').contains(&c) {
            c as u32 - '0' as u32
        } else if ('A'..'G').contains(&c) {
            c as u32 - 'A' as u32 + 10
        } else {
            unreachable!()
        }
    }).map(|i| {
        [(i & 0x8) != 0,
         (i & 0x4) != 0,
         (i & 0x2) != 0,
         (i & 0x1) != 0]
    }).flatten().enumerate().peekable();

    parse_packet(&mut it)
}

fn sum_versions(p: &Packet) -> usize {
    (match &p.contents {
        Contents::Lit(_) => 0,
        Contents::Op(_, v) => v.iter().map(sum_versions).sum(),
    }) + p.version
}

fn eval(p: &Packet) -> usize {
    match &p.contents {
        &Contents::Lit(x) => x,
        Contents::Op(op, v) => {
            let mut it = v.iter().map(eval);
            match op {
                Op::Sum => it.sum(),
                Op::Product => it.product(),
                Op::Min => it.min().unwrap(),
                Op::Max => it.max().unwrap(),
                _ => {
                    let first = it.next().unwrap();
                    let second = it.next().unwrap();
                    (match op {
                        Op::Gt => first > second,
                        Op::Lt => first < second,
                        Op::Eq => first == second,
                        _ => unreachable!(),
                    }) as usize
                }
            }
        }
    }
}

fn first_part(s: &str) -> usize {
    sum_versions(&parse(s))
}

fn second_part(s: &str) -> usize {
    eval(&parse(s))
}
