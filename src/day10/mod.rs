#![allow(unused)]

use crate::utils::read_file;

// https://adventofcode.com/2022/day/10
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let content = parse_input();

    println!("{:?}", content);
}

fn parse2() {}

fn parse_input() -> String {
    let content = read_file("day10/input.txt").unwrap();

    content
}
