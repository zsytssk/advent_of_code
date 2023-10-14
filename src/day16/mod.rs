#![allow(unused)]
use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    cmp::Ordering,
};

use crate::utils::read_file;

mod value;

use value::*;

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
    let path: Vec<String> = Vec::new();
    let path = find_path(cur_value.unwrap(), &map, path);

    let sum = path.iter().fold(0, |acc, x| {
        let ele = map.get_value(x).unwrap();
        acc + ele.rate
    });
    println!("sum:{:?}\npath={:?}", sum, path);
}

fn find_path<'a>(
    cur_value: Ref<Value>,
    map: &Map,
    mut path: Vec<String>,
) -> Vec<String> {
    let mut arr = cur_value.to.clone();
    arr.sort_by(|a, b| {
        let a_next_wrap = map.get_value(a);
        let b_next_wrap = map.get_value(b);
        if a_next_wrap.is_none() || b_next_wrap.is_none() {
            return Ordering::Equal;
        }
        let (a_next, b_next) = (a_next_wrap.unwrap(), b_next_wrap.unwrap());
        a_next.rate.cmp(&b_next.rate)
    });

    for item in arr.iter().rev() {
        if path.contains(&item) {
            continue;
        }
        path.push(item.clone());
        let last_ele = map.get_value(&item);
        path = find_path(last_ele.unwrap(), map, path);
    }

    // for item in to {
    //     let next_wrap = map.get_value(item);
    //     if next_wrap.is_none() {
    //         continue;
    //     }
    //     let next_value = next_wrap.unwrap();
    //     next_value.rate
    // }

    path
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
