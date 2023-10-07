#![allow(unused)]
use std::{
    cell::{Ref, RefCell, RefMut},
    string,
    time::Instant,
};

use crate::utils::read_file;

mod map;
use map::*;

// 如何求最短路径
// 到某个点的最短距离
// 怎么判断每一个步数的 效率？ -> 每走3步找一个优先级 -> 卡死了这个优先级降低
// 贪婪算法 ｜ 迟钝算法
// 朝向他的为优先级高
// 如何同时往多个方向寻找 + 如何
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let now = Instant::now();
    let map = parse_input();

    let mut start_wap = None;
    let mut end_wap = None;

    for y in 0..map.y {
        for x in 0..map.x {
            let item = map.get_point(x, y);
            if item.unwrap().borrow().has_letter("S") {
                start_wap = item;
                continue;
            }
            if item.unwrap().borrow().has_letter("E") {
                end_wap = item;
            }
        }
    }
    if (start_wap.is_none() || end_wap.is_none()) {
        panic!("start or end not found!");
    }

    let (start, end) = (start_wap.unwrap(), end_wap.unwrap());
    let (find, step) = find_end_len(
        start,
        &map,
        &vec![(start.borrow().x, start.borrow().y, String::from("S"))],
    );

    println!(
        "find={:?} step={:?} cost_time={:?}",
        end,
        find,
        now.elapsed()
    );
}

fn find_end_len(
    pos_wrap: &RefCell<Point>,
    map: &Map,
    path: &Vec<(usize, usize, String)>,
) -> (bool, usize) {
    let pos = pos_wrap.borrow();

    if pos.is_end() {
        return (true, path.len());
    }

    let mut find_arr = Vec::new();
    for dir in pos.get_move_dir().iter() {
        let (x, y) = get_dir_pos(pos_wrap.borrow(), dir, map).unwrap();
        if path_has_point(&path, (x, y)) {
            continue;
        }
        let next_pos = map.get_point(x, y).unwrap();
        let mut clone_path = path.clone();

        clone_path.push((x, y, String::from(&next_pos.borrow().letter)));
        let (find, step) = find_end_len(next_pos, map, &clone_path);
        if find == true {
            find_arr.push(step)
        }
    }

    if find_arr.len() != 0 {
        find_arr.sort_by(|a, b| a.cmp(&b));
        return (true, find_arr[0]);
    }

    return (false, 0);
}

fn path_has_point(
    path: &Vec<(usize, usize, String)>,
    point: (usize, usize),
) -> bool {
    for (x, y, _) in path {
        if *x == point.0 && *y == point.1 {
            return true;
        }
    }
    return false;
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
