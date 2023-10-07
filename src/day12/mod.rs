#![allow(unused)]
use crate::utils::read_file;

mod point;
use point::*;

// 如何求最短路径
// 到某个点的最短距离
// 怎么判断每一个步数的 效率？
// 朝向他的为优先级高
// 如何同时往多个方向寻找 + 如何
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let map = parse_input();

    let mut start = Point::init();
    let mut end = Point::init();

    for i in 0..map.len() {
        let line = &map[i];
        for j in 0..line.len() {
            if line[j] == "S" {
                start.update_pos(i, j);
            }
            if line[j] == "E" {
                end.update_pos(i, j);
            }
        }
    }

    println!("{:?} {:?}", start, end);
}

fn parse_input() -> Vec<Vec<String>> {
    let content = read_file("day12/demo.txt").unwrap();

    content
        .split("\n")
        .map(|line| {
            line.split("")
                .filter(|&item| item != "")
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
