#![allow(unused)]
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    time::Instant,
};

use crate::{day16::utils::get_short_path, utils::read_file};

mod map;
mod test;
mod utils;

use map::*;

type PathMap = HashMap<Vec<String>, (usize, i32)>;

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

    let loop_path_map: PathMap = HashMap::new();
    let short_path = get_short_path(&path_arr, &map);

    let cur_path = vec!["AA".to_string()];
    loop {
        let cur_keys = cur_path.clone();
    }

    println!("now:{:?}\nmap:{:?}", now.elapsed(), short_path);
}

fn calc_top_path(
    loop_path_map: &mut PathMap,
    short_path: &HashMap<(String, String), usize>,
    map: &Switches,
) -> Vec<(Vec<(String, bool)>)> {
    todo!()
}

fn get_next_keys(cur_key: &Vec<String>, map: &Switches) -> Vec<String> {}

fn parse_input() -> Switches {
    let content = read_file("day16/input.txt").unwrap();

    let list = content
        .split("\n")
        .map(Switch::from_str)
        .map(|item| RefCell::new(item))
        .collect::<Vec<_>>();

    Switches::new(list)
}
