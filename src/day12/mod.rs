#![allow(unused)]
use std::cell::{Ref, RefCell, RefMut};

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

    let mut start = None;
    let mut end = None;

    for y in 0..map.y {
        for x in 0..map.x {
            let item = map.get_point(x, y).unwrap().borrow();
            if item.has_letter("S") {
                start = Some(item);
                continue;
            }
            if item.has_letter("E") {
                end = Some(item);
            }
        }
    }

    // println!("{:?}", map);
    println!("{:?} {:?}", start, end);
}

fn parse_input() -> Map {
    let content = read_file("day12/demo.txt").unwrap();

    let map_str = content
        .split("\n")
        .map(|line| {
            line.split("")
                .filter(|&item| item != "")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut map = Map::new(map_str[0].len(), map_str.len());
    for y in 0..map_str.len() {
        let line_str = &map_str[y];
        for x in 0..line_str.len() {
            let p = Point::init(x, y, String::from(line_str[x]));
            map.add_point(p);
        }
    }

    for y in 0..map_str.len() {
        let line_str = &map_str[y];
        for x in 0..line_str.len() {
            match map.get_point(x, y) {
                None => continue,
                Some(p) => check_move(p, &map),
            }
        }
    }

    map
}

fn check_move(mut point: &RefCell<Point>, map: &Map) {
    let arr = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];
    let mut dir_arr = Vec::new();
    for dir in arr {
        let diff_pos = get_dir_pos(point.borrow(), &dir, map);
        match diff_pos {
            Some(p) => match map.get_point(p.0, p.1) {
                None => {}
                Some(diff_point) => {
                    if (point.borrow().can_move(diff_point.borrow())) {
                        dir_arr.push(dir.clone());
                    }
                }
            },
            None => {}
        }
    }

    point.borrow_mut().set_move_dir(dir_arr);
}

fn get_dir_pos(
    point: Ref<Point>,
    dir: &Dir,
    map: &Map,
) -> Option<(usize, usize)> {
    match dir {
        Dir::Left => {
            if point.x == 0 {
                return None;
            }
            return Some((point.x - 1, point.y));
        }
        Dir::Right => {
            if point.x == map.x - 1 {
                return None;
            }
            return Some((point.x + 1, point.y));
        }
        Dir::Up => {
            if point.y == 0 {
                return None;
            }
            return Some((point.x, point.y - 1));
        }
        Dir::Down => {
            if point.y == map.y - 1 {
                return None;
            }
            return Some((point.x, point.y + 1));
        }
    }
}
