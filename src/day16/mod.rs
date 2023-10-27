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
    let path_arr = map
        .list
        .iter()
        .filter(|item| item.borrow().rate != 0 || item.borrow().name == "AA")
        .collect::<Vec<_>>();

    let mut loop_path_map: PathMap = HashMap::new();
    let mut complete_path_map: PathMap = HashMap::new();
    let short_path = get_short_path(&path_arr, &map);
    let first_key = MapKey::new(vec!["AA".to_string()], 30);

    let mut cur_paths = vec![first_key];
    loop {
        for path in cur_paths.iter() {
            let cur_score = match loop_path_map.get(path) {
                None => 0,
                Some(t) => t.clone(),
            };
            let key = path.clone();
            let next_info_list =
                path.get_next_keys(&path_arr, &short_path, &map);

            if next_info_list.len() != 0 {
                loop_path_map.remove(path);
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
) -> Vec<(Vec<(String, bool)>)> {
    let mut path_arr: Vec<_> = complete_path_map.iter().collect();
    path_arr.sort_by(|a, b| {
        a.0
    })
    todo!()
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
