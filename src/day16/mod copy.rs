#![allow(unused)]
use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    cmp::Ordering,
    collections::HashMap,
};

use crate::utils::read_file;

mod switch;

use switch::*;

type PathKey = Vec<(String, bool)>;

type PathMap = HashMap<PathKey, (usize, i32)>;

pub fn parse() {
    // test();
    parse1();
    // parse2();
}

// fn test() {
//     let map = parse_input();
//     // let path = "AA-DD-AA-BB-AA-II-JJ-II-AA-DD-EE-FF-GG-HH-GG-FF-EE-DD-CC";
//     let path = "AA-DD-CC-BB-AA-II-JJ-II-AA-DD-EE-FF-GG-HH-GG-FF-EE-DD-CC";
//     let path_arr = path.split("-").collect::<Vec<_>>();

//     let mut pass_path = String::from("");
//     let mut cur_time = 30;
//     let mut score = 0 as usize;
//     for name in path_arr.iter() {
//         let mut value = map.get_value(name).unwrap();

//         if value.rate > 0 && value.is_open == false {
//             cur_time -= 1;
//             score += value.rate as usize * cur_time;
//             value.set_open(true)
//         }
//         println!(
//             "item={:?}| cur_time={:?} | rate={} | cur_score={:?}",
//             name, cur_time, value.rate, score
//         );
//         cur_time -= 1;
//         pass_path = format!("{}-{}", pass_path, name);
//     }

//     println!("score={:?}\npass_path={:?}", score, pass_path)
// }

fn parse1() {
    let map = parse_input();
    let mut path_map: PathMap = HashMap::new();
    let first_key = vec![(String::from("AA"), false)];
    path_map.insert(first_key.clone(), (0, 30));
    let mut cur_arr = vec![first_key];

    for i in 0..6 {
        for cur_path in cur_arr.iter() {
            find_path(cur_path, &map, &mut path_map, 6);
        }
        let path_arr = get_top_path(&path_map);
        println!("path_map:{:?}", path_arr[0]);
        cur_arr = path_arr.into_iter().map(|item| item.0).collect();
    }

    let path_arr = get_top_path(&path_map);
    println!("path_map:{:?}", path_arr[0]);
}

fn get_top_path(path_map: &PathMap) -> Vec<(Vec<(String, bool)>, usize)> {
    let mut vec = path_map.iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    vec.iter().map(|item| (item.0.clone(), item.1 .0)).collect()
}

fn find_path(
    cur_path: &PathKey,
    map: &Switches,
    path_map: &mut PathMap,
    mut time_space: i32,
) {
    if time_space <= 0 {
        return;
    }
    let cur_info = match path_map.get(cur_path) {
        Some(info) => info.clone(),
        None => return,
    };

    let cur_name = get_last_name(cur_path).unwrap();
    let cur_value = match map.get_value(&cur_name) {
        Some(item) => item,
        None => panic!("cant find item name={}", cur_name),
    };
    let mut arr = cur_value
        .to
        .iter()
        .map(|item| map.get_value(item))
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect::<Vec<_>>();

    arr.sort_by(|a, b| {
        let a_rate = match has_opened(&a.name, cur_path) {
            true => 0,
            false => a.rate,
        };
        let b_rate = match has_opened(&b.name, cur_path) {
            true => 0,
            false => b.rate,
        };
        b_rate.cmp(&a_rate)
    });

    let mut find_deep = false;
    for mut item in arr.into_iter() {
        let has_opened = has_opened(&item.name, cur_path);

        let (mut cur_score, mut cur_time) = cur_info;
        let mut key = cur_path.clone();

        let rate = item.rate as usize;
        if rate <= 0 || has_opened {
            key.push((item.name.clone(), false));
            cur_time -= 1;
            time_space -= 1;
        }
        if rate > 0 && has_opened == false {
            cur_time -= 1;
            time_space -= 1;
            cur_score += rate * cur_time as usize;
        }
        if cur_time < 0 {
            continue;
        }
        find_deep = true;
        path_map.insert(key.clone(), (cur_score, cur_time));
        find_path(&key, map, path_map, time_space);
    }

    if find_deep {
        path_map.remove(cur_path);
    }
}

fn get_last_name(path: &PathKey) -> Option<String> {
    match path.iter().last() {
        Some(item) => Some(item.0.clone()),
        None => None,
    }
}
fn has_opened(name: &String, path: &PathKey) -> bool {
    match path.iter().find(|item| &item.0 == name && item.1) {
        Some(_) => true,
        None => false,
    }
}

fn parse_input() -> Switches {
    let content = read_file("day16/demo.txt").unwrap();

    let list = content
        .split("\n")
        .map(Switch::from_str)
        .map(|item| RefCell::new(item))
        .collect::<Vec<_>>();

    Switches::new(list)
}
