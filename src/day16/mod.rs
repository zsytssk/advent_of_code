#![allow(unused)]
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    time::Instant,
};

use crate::{day16::utils::get_short_path, utils::read_file};

mod map;
mod utils;

use map::*;

type PathMap = HashMap<MapKey, usize>;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let now = Instant::now();
    let map = parse_input();
    let path_arr = map.get_rate_keys();

    let mut loop_path_map: PathMap = HashMap::new();
    let mut complete_path_map: PathMap = HashMap::new();
    let short_path = get_short_path(&path_arr, &map);
    let first_key = MapKey::new(vec!["AA".to_string()], 30, path_arr.len());

    let mut cur_paths = vec![first_key];
    loop {
        for path in cur_paths.iter_mut() {
            let cur_score = match loop_path_map.get(path) {
                None => 0,
                Some(t) => t.clone(),
            };
            let key = path.clone();
            let next_info_list =
                path.get_next_keys(&path_arr, &short_path, &map);

            if next_info_list.len() != 0 {
                loop_path_map.remove(path);
            } else {
                path.set_time(0);
            }
            for (next_key, next_score) in next_info_list {
                loop_path_map.insert(next_key, cur_score + next_score);
            }
        }
    }

    println!("test:>2{:?}", loop_path_map);
}

fn calc_top_path(
    loop_path_map: &mut PathMap,
    complete_path_map: &mut PathMap,
    short_path: &HashMap<(String, String), usize>,
    map: &Switches,
) -> Vec<(MapKey)> {
    let mut path_arr: Vec<_> = complete_path_map.iter().collect();
    path_arr.sort_by(|a, b| b.1.cmp(&a.1));
    let big_num = path_arr[0].1;

    let mut arr = Vec::new();
    loop_path_map.retain(|key, score| {
        if key.is_complete() {
            if *score > *big_num {
                complete_path_map.insert(key.clone(), *score);
            }
            return false;
        }

        let big_score = key.get_max_score(short_path, map);
        if big_score + *score < *big_num {
            return false;
        }
        arr.push((key.clone(), score.clone()));
        return true;
    });

    if arr.len() == 0 {
        return vec![];
    }

    arr.sort_by(|a, b| {
        let rate_cmp = b.1.cmp(&a.1);
        if rate_cmp != std::cmp::Ordering::Equal {
            return rate_cmp;
        }
        b.0.get_time().cmp(&a.0.get_time())
        // b.0.sum_time().cmp(&a.0.sum_time())
        // rate_cmp
    });

    let big_score = arr[0].1;
    let big_time = arr[0].0.get_time();

    arr.into_iter()
        .filter(|item| item.1 == big_score && item.0.get_time() == big_time)
        .map(|item| item.0)
        // .take(200)
        .collect::<Vec<_>>()
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
