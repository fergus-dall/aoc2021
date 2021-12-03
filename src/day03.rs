use std::fs;
use std::string::String;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", first_part(&str));
    println!("{}", second_part(&str));
}

fn count(v: &Vec<&str>) -> Vec<usize> {
    let mut ret = Vec::with_capacity(v[0].len());
    for _ in 0..v[0].len() {
        ret.push(0);
    }

    for s in v {
        let mut idx = 0;
        for c in s.chars() {
            match c {
                '1' => ret[idx] += 1,
                '0' => (),
                _ => panic!(),
            }
            idx += 1;
        }
    }

    ret
}

fn first_part(s: &str) -> isize {
    let lines = s.split_terminator("\n").collect();

    let count = count(&lines);

    let threshold = lines.len() / 2;
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in count {
        gamma *= 2;
        epsilon *= 2;
        if i > threshold {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    gamma * epsilon
}

fn str_to_int(s: &str) -> isize {
    let mut ret = 0;
    for c in s.chars() {
        ret *= 2;
        match c {
            '1' => ret += 1,
            _ => (),
        }
    }

    ret
}

fn second_part(s: &str) -> isize {
    let mut co2 : Vec<&str> = s.split_terminator("\n").collect();
    let mut o_gen = co2.clone();

    for idx in 0..co2[0].len() {
        let mut new_co2 = Vec::new();

        let count = count(&co2);
        let threshold = (co2.len()+1) / 2;
        for s in co2 {
            match s.as_bytes()[idx] {
                48 => if count[idx] >= threshold {new_co2.push(s)} ,
                49 => if count[idx] < threshold {new_co2.push(s)},
                _ => panic!(),
            }
        }

        co2 = new_co2;

        if co2.len() == 1 {
            break
        }
    }

    for idx in 0..o_gen[0].len() {
        let mut new_o_gen = Vec::new();

        let count = count(&o_gen);
        let threshold = (o_gen.len()+1) / 2;
        for s in o_gen {
            match s.as_bytes()[idx] {
                48 => if count[idx] < threshold {new_o_gen.push(s)} ,
                49 => if count[idx] >= threshold {new_o_gen.push(s)},
                _ => panic!(),
            }
        }

        o_gen = new_o_gen;

        if o_gen.len() == 1 {
            break
        }
    }

    str_to_int(o_gen[0]) * str_to_int(co2[0])
}

