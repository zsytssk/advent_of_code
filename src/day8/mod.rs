#![allow(unused)]

use crate::utils::read_file;

// https://adventofcode.com/2022/day/8
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let content = parse_input();
    let arr = content
        .split("\n")
        .map(|line| {
            line.split("").collect::<Vec<&str>>()
            // line.split("")
            //     .map(|item| item.parse::<u32>().unwrap())
            //     .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", arr);
}

fn parse2() {}

fn parse_input() -> String {
    let content = read_file("day7/demo.txt").unwrap();

    content
}
