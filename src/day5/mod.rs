#![allow(unused)]
use crate::utils::read_file;

// https://adventofcode.com/2022/day/5
pub fn parse() {
    parse1();
    parse2();
}

fn parse1() {
    let content = read_file("day5/input.txt").unwrap();
    println!("{}", content)
}

fn parse2() {}
