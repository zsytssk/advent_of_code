#![allow(unused)]
use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    cmp::Ordering,
    collections::HashMap,
};

use crate::utils::read_file;

mod value;

use value::*;

type PathMap = HashMap<String, (usize, usize)>;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let map = parse_input();
    let cur_value = map.get_value("AA");
    if cur_value.is_none() {
        panic!();
    }
    let cur_path = "AA";
    let mut path_map: PathMap = HashMap::new();
    path_map.insert(String::from(cur_path), (0, 30));
    find_path(cur_value.unwrap(), &map, cur_path, &mut path_map);

    println!("path_map:{:?}", path_map);
}

fn find_path<'a>(
    cur_value: Ref<Value>,
    map: &Map,
    cur_path: &str,
    path_map: &mut PathMap,
) {
    let cur_info = match path_map.get(cur_path) {
        Some(info) => info.clone(),
        None => return,
    };

    for item in cur_value.to.iter().rev() {
        match cur_path.find(item) {
            Some(_) => continue,
            None => {
                let value = match map.get_value(&item) {
                    Some(value) => value,
                    None => return,
                };

                let mut cur_score = cur_info.0;
                let key = format!("{}-{}", cur_path, item);
                let mut cur_time = cur_info.1 - 1;
                if cur_time <= 0 {}
                let rate = value.rate as usize;
                if rate > 0 {
                    cur_time -= 1;
                    cur_score += rate * cur_time;
                }
                if cur_time <= 0 {
                    continue;
                }
                path_map.insert(key.clone(), (cur_score, cur_time));
                find_path(value, map, &key, path_map);
            }
        }
    }

    // for item in to {
    //     let next_wrap = map.get_value(item);
    //     if next_wrap.is_none() {
    //         continue;
    //     }
    //     let next_value = next_wrap.unwrap();
    //     next_value.rate
    // }
}

fn parse_input() -> Map {
    let content = read_file("day16/demo.txt").unwrap();

    let list = content
        .split("\n")
        .map(Value::from_str)
        .map(|item| RefCell::new(item))
        .collect::<Vec<_>>();

    Map::new(list)
}
