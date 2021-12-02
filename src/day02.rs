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
    let mut horz = 0;
    let mut depth = 0;

    for part in s.split("\n") {
        let idx = match part.find(" ") {
            Some(i) => i,
            None => continue,
        };

        let n = match isize::from_str(part.split_at(idx+1).1) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if part.starts_with("up") {
            depth -= n;
        } else if part.starts_with("down") {
            depth += n;
        } else if part.starts_with("forward") {
            horz += n;
        } else {
            continue
        }
    };

    horz*depth
}

fn second_part(s: &str) -> isize {
    let mut horz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for part in s.split("\n") {
        let idx = match part.find(" ") {
            Some(i) => i,
            None => continue,
        };

        let n = match isize::from_str(part.split_at(idx+1).1) {
            Ok(i) => i,
            Err(_) => continue,
        };

        if part.starts_with("up") {
            aim -= n;
        } else if part.starts_with("down") {
            aim += n;
        } else if part.starts_with("forward") {
            horz += n;
            depth += aim * n;
        } else {
            continue
        }
    };

    horz*depth
}
