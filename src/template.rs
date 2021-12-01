use std::fs;
use std::string::String;

fn main() {
    let filename = format!("inputs/{}.txt", module_path!());
    let text = fs::read(filename).ok().expect("Couldn't open file");
    let str = String::from_utf8(text).ok().expect("Could parse UTF8 from file");

    println!("{}", do_stuff());
}

fn do_stuff() -> u32 {
    0
}
