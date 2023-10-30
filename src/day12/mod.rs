#![allow(unused)]
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    string,
    time::Instant,
};

use crate::utils::read_file;

mod map;
mod save;
mod utils;
use map::*;

use utils::*;

// https://adventofcode.com/2022/day/12#part2
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
    let mut loop_paths = vec![(start.borrow(), 0, 0)];
    let mut cur_paths = vec![(start.borrow(), 0, 0)];

    let mut find_item = None;

    loop {
        for (index, (item, step, _)) in cur_paths.iter().enumerate() {
            let next_paths = get_next_step(item, &map);
            if next_paths.len() == 0 {
                continue;
            }

            let next_paths = next_paths
                .into_iter()
                .map(|item| {
                    let dis = end.borrow().distance(&item);
                    let new_step = step + 1;
                    (item, step + 1, new_step + dis)
                })
                .collect::<Vec<_>>();

            for item in next_paths.iter() {
                if end.borrow().is_same(&item.0) {
                    find_item = Some((&item.0, item.1));
                    break;
                }
            }

            loop_paths.extend(next_paths);
        }

        cur_paths = calc_top_path(&mut loop_paths, end.borrow());

        println!("find={:?} cost_time={:?}", cur_paths.len(), now.elapsed());
    }

    let find_item = find_item.unwrap();

    println!(
        "find={:?} step={:?}  cost_time={:?}",
        find_item,
        find_item.1,
        now.elapsed()
    );
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
        let diff_pos = get_dir_pos(&point.borrow(), &dir, map);
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
    point: &Ref<Point>,
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
