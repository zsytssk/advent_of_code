#![allow(unused)]
use std::time::Instant;

use crate::utils::read_file;

use self::cube::Cube;

mod cube;

pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let now = Instant::now();
    let list = parse_input();
    let mut overlap_num = 0;
    for i in 0..(list.len() - 1) {
        for j in i + 1..list.len() {
            let cube1 = &list[i];
            let cube2 = &list[j];
            if cube1.is_overlap(cube2) {
                overlap_num += 1;
            }
        }
    }
    let res = 6 * list.len() - 2 * overlap_num;
    println!("time={:?}|res={:?}", now.elapsed(), res);
}

fn parse2() {
    let now = Instant::now();
    let list = parse_input();
    let mut overlap_num = 0;
    for i in 0..(list.len() - 1) {
        for j in i + 1..list.len() {
            let cube1 = &list[i];
            let cube2 = &list[j];
            if cube1.is_overlap(cube2) {
                overlap_num += 1;
            }
        }
    }
    let res = 6 * list.len() - 2 * overlap_num;
    println!("time={:?}|res={:?}", now.elapsed(), res);
}

fn parse_input() -> Vec<Cube> {
    let content = read_file("day18/demo.txt").unwrap();

    content
        .split("\n")
        .map(|item| {
            let cor = item
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Cube::new(cor[0], cor[1], cor[2])
        })
        .collect::<Vec<_>>()
}
