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
