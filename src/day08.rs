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
use std::convert::TryInto;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

struct Display {
    examples: [[bool; 7]; 10],
    display: [[bool; 7]; 4],
}

impl FromStr for Display {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ex, curr) = s.split_once('|').ok_or("Couldn't find '|'")?;

        let mut examples = Vec::new();
        for s in ex.split_whitespace() {
            let mut r = Vec::new();
            for i in 0..7 {
                let c = char::from_u32('a' as u32 + i).unwrap();
                r.push(s.contains(c));
            }
            let r = r.try_into().unwrap();
            examples.push(r);
        }
        let examples = examples.try_into().or(
            Err("Didn't find 10 examples!"))?;

        let mut display = Vec::new();
        for s in curr.split_whitespace() {
            let mut r = Vec::new();
            for i in 0..7 {
                let c = char::from_u32('a' as u32 + i).unwrap();
                r.push(s.contains(c));
            }
            let r = r.try_into().unwrap();
            display.push(r);
        }
        let display = display.try_into().or(
            Err("Didn't find 4 digits!"))?;

        Ok(Display {examples, display})
    }
}

impl Display {
    fn exclude(dis1: &[bool; 7], dis2: &[bool; 7]) -> [bool; 7] {
        dis1.iter().zip(dis2.iter()).map(|(a,b)| *a && !b)
            .collect::<Vec<bool>>().try_into().unwrap()
    }

    fn count(dis: &[bool; 7]) -> usize {
        dis.iter().fold(0, |x,y| x+(*y as usize))
    }

    // Mapping goes from wire -> segment
    fn solve(&self) -> [usize; 7] {
        let two_segment = self.examples.iter()
            .find(|&x| Display::count(x) == 2).unwrap();
        let three_segment = self.examples.iter()
            .find(|&x| Display::count(x) == 3).unwrap();
        let four_segment = self.examples.iter()
            .find(|&x| Display::count(x) == 4).unwrap();

        // Find the (cf) segment pair, as the two segments lit up
        // on the 2-segment display. We don't yet know which is which
        // though.
        let mut iter = two_segment.iter()
            .enumerate().filter(|(_,&val)| val);
        let cf_segments = (
            iter.next().unwrap().0,
            iter.next().unwrap().0
        );

        // Find the 'a' segment. This is the segment lit up on the only
        // 3-segment display, that is not lit on the only 2-segment
        // display.
        let a_segment = Display::exclude(three_segment, two_segment)
            .iter().enumerate().find(|(_,&val)| val).unwrap().0;

        // Find the (bd) segment pair, as the two segments lit up on
        // the 4-segment display but not the 2-segment display.
        let diff = Display::exclude(four_segment, two_segment);
        let mut iter = diff.iter().enumerate().filter(|(_,&val)| val);
        let bd_segments = (
            iter.next().unwrap().0,
            iter.next().unwrap().0
        );

        // Find the (eg) segment as the remaining two segments.
        let mut iter = (0..7).filter(|&i| {
            i != a_segment &&
                i != cf_segments.0 &&
                i != cf_segments.1 &&
                i != bd_segments.0 &&
                i != bd_segments.1
        });
        let eg_segments = (
            iter.next().unwrap(),
            iter.next().unwrap()
        );

        // For two of these pairs, one element is lit on all three
        // 5-segment displays, one element is lit for only 1.
        let check_single = |idx| {
            self.examples.iter()
                .filter(|&x| x[idx] && Display::count(x) == 5).count() == 1
        };

        let (b_segment, d_segment);
        if check_single(bd_segments.0) {
            b_segment = bd_segments.0;
            d_segment = bd_segments.1;
        } else {
            b_segment = bd_segments.1;
            d_segment = bd_segments.0;
        }

        let (e_segment, g_segment);
        if check_single(eg_segments.0) {
            e_segment = eg_segments.0;
            g_segment = eg_segments.1;
        } else {
            e_segment = eg_segments.1;
            g_segment = eg_segments.0;
        }

        // For (cf) we use a different trick. Wire 'f' is lit on all
        // 6-segment displays, while 'c' is only lit on 2 of them.
        let (c_segment, f_segment);
        if self.examples.iter()
            .filter(|&x| x[cf_segments.0] &&
                    Display::count(x) == 6).count() == 2 {
                c_segment = cf_segments.0;
                f_segment = cf_segments.1;
            } else {
                c_segment = cf_segments.1;
                f_segment = cf_segments.0;
            }

        [a_segment, b_segment, c_segment, d_segment,
         e_segment, f_segment, g_segment]
    }

    fn map(&self, map: [usize; 7]) -> [usize; 4] {
        let mut result = Vec::new();

        for i in 0..4 {
            let count = self.display[i].iter().fold(0, |x,y| x+(*y as usize));
            result.push(match count {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => {
                    if self.display[i][map[1]] {
                        // wire b is set
                        5
                    } else if self.display[i][map[4]] {
                        // wire e is set
                        2
                    } else {
                        3
                    }
                },
                6 => {
                    if !self.display[i][map[3]] {
                        // wire d is unset
                        0
                    } else if !self.display[i][map[2]] {
                        // wire c is unset
                        6
                    } else if !self.display[i][map[4]] {
                        // wire 4 is unset
                        9
                    } else {
                        // Invalid code
                        panic!("")
                    }
                },
                7 => 8,
                _ => panic!(""),
            })
        }

        result.try_into().unwrap()
    }
}

fn first_part(s: &str) -> usize {
    let mut result = 0;
    for line in s.split_terminator('\n') {
        let display: Display = line.parse().expect("Failed to parse line");
        let map = display.solve();
        result += display.map(map).iter()
            .filter(|&&i| i == 1 || i == 4 || i == 7 || i == 8).count();
    }
    result
}

fn second_part(s: &str) -> usize {
    let mut result = 0;
    for line in s.split_terminator('\n') {
        let display: Display = line.parse().expect("Failed to parse line");
        let map = display.solve();
        result += display.map(map).iter().fold(0, |x,y| (x*10)+y);
    }
    result
}
