#![allow(unused)]
use crate::utils::read_file;

mod value;

use value::*;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let content = parse_input();
    println!("{:?}", content);
}

fn parse_input() -> Vec<Value> {
    let content = read_file("day16/demo.txt").unwrap();

    content.split("\n").map(Value::from_str).collect::<Vec<_>>()
}
