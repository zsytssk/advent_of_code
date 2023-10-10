#![allow(unused)]
use crate::utils::read_file;
mod map;

use map::*;

// 要先画一个map
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let content = parse_input();
    println!("{:?}", content);
}

fn parse_input() -> Vec<Path> {
    let content = read_file("day14/demo.txt").unwrap();

    content
        .split("\n")
        .map(|line| {
            let points =
                line.split(" -> ").map(Point::from_str).collect::<Vec<_>>();
            Path::new(points)
        })
        .collect::<Vec<_>>()
}
