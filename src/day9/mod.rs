#![allow(unused)]

use crate::utils::read_file;

// https://adventofcode.com/2022/day/8
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let arr = parse_input();

    println!("{:?}", arr.len());
}

fn parse2() {}

fn parse_input() -> String {
    let content = read_file("day9/input.txt").unwrap();

    content
}
