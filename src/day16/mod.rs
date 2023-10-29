#![allow(unused)]
use std::{
    cell::{Ref, RefCell},
    cmp,
    collections::HashMap,
    time::Instant,
};

use crate::{day16::utils::get_short_path, utils::read_file};

mod map;
mod utils;

use map::*;

type PathMap = HashMap<MapKey, usize>;
type PathList = Vec<(MapKey, usize, usize)>;
pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let now = Instant::now();
    let map = parse_input();
    let path_arr = map.get_rate_keys();

    let mut loop_paths: PathList = vec![];
    let mut complete_paths: PathList = vec![];
    let short_path = get_short_path(&path_arr, &map);
    let first_key = MapKey::new(vec!["AA".to_string()], 30, 0, path_arr.len());

    let mut cur_paths = vec![(first_key, 0, 0)];
    let type_path = TypePath::Type1;
    loop {
        let mut remove_index_list = vec![];
        let mut add_list = vec![];
        for (index, (path, cur_score, _)) in cur_paths.iter_mut().enumerate() {
            let key = path.clone();
            let next_info_list =
                path.get_next_keys(&type_path, &path_arr, &short_path, &map);

            if next_info_list.len() != 0 {
                remove_index_list.push(index);
            } else {
                path.set_time(&type_path, 0);
            }

            for (next_key, next_score, max_score) in next_info_list {
                add_list.push((next_key, *cur_score + next_score, max_score));
            }
        }

        remove_index_list.sort_by(|a, b| b.cmp(&a));
        for index in remove_index_list {
            cur_paths.remove(index);
        }
        cur_paths.extend(add_list);

        if cur_paths.len() == 0 && loop_paths.len() == 0 {
            break;
        }
        calc_top_path(&mut cur_paths, &mut loop_paths, &mut complete_paths);
    }

    println!("time={:?}\nres={:?}", now.elapsed(), complete_paths[0]);
}

fn parse2() {
    let now = Instant::now();
    let map = parse_input();
    let path_arr = map.get_rate_keys();
    let short_path = get_short_path(&path_arr, &map);
    let first_key = MapKey::new(vec!["AA".to_string()], 26, 26, path_arr.len());

    let mut complete_paths: PathList = vec![];

    // for i in 0..path_arr.len() + 1 {
    // let mut type1_num = (i) as i32;
    // let mut type2_num = (path_arr.len() - i) as i32;
    let mut type1_num = (4) as i32;
    let mut type2_num = (4) as i32;

    println!(
        "step: all={:?} | type1_num={:?} | type2_num={:?}",
        path_arr.len(),
        type1_num,
        type2_num
    );
    let mut cur_paths = vec![(first_key.clone(), 0, 0)];
    let mut type_path = TypePath::Type1;
    let mut loop_paths: PathList = vec![];
    loop {
        type2_num -= 1;
        if type2_num >= 0 {
            type_path = TypePath::Type2
        } else {
            type_path = TypePath::Type1
        }

        let mut remove_index_list = vec![];
        let mut add_list = vec![];
        for (index, (path, cur_score, _)) in cur_paths.iter_mut().enumerate() {
            let key = path.clone();
            let next_info_list =
                path.get_next_keys(&type_path, &path_arr, &short_path, &map);

            if next_info_list.len() != 0 {
                remove_index_list.push(index);
            } else {
                path.set_time(&type_path, 0);
            }

            for (next_key, next_score, max_score) in next_info_list {
                add_list.push((next_key, *cur_score + next_score, max_score));
            }
        }

        remove_index_list.sort_by(|a, b| b.cmp(&a));
        for index in remove_index_list {
            cur_paths.remove(index);
        }
        cur_paths.extend(add_list);

        if cur_paths.len() == 0 && loop_paths.len() == 0 {
            break;
        }
        calc_top_path(&mut cur_paths, &mut loop_paths, &mut complete_paths);
    }
    // }

    // println!("time={:?}\nres={:?}", now.elapsed(), complete_paths[0]);
    println!("time={:?}\nres={:?}", now.elapsed(), complete_paths[0]);
}

fn calc_top_path(
    cur_paths: &mut PathList,
    loop_paths: &mut PathList,
    complete_paths: &mut PathList,
) {
    let big_num = match complete_paths.get(0) {
        None => 0,
        Some(t) => t.1.clone(),
    };

    let max_len = 1000;
    if cur_paths.len() == 0 {
        let num = cmp::min(max_len, loop_paths.len());
        let add_list = loop_paths.split_off(loop_paths.len() - num);
        cur_paths.extend(add_list);
    }

    cur_paths.retain(|(key, score, max_core)| {
        if key.is_complete() {
            if *score > big_num {
                complete_paths.insert(0, (key.clone(), *score, 0));
            }
            return false;
        }

        if max_core + *score < big_num {
            return false;
        }
        return true;
    });

    if cur_paths.len() == 0 {
        return;
    }

    cur_paths.sort_by(|a, b| {
        let rate_cmp = b.1.cmp(&a.1);
        if rate_cmp != std::cmp::Ordering::Equal {
            return rate_cmp;
        }
        b.0.rest_time().cmp(&a.0.rest_time())
    });

    let big_score = cur_paths[0].1;
    let big_time = cur_paths[0].0.rest_time();

    cur_paths.retain(|item| {
        if item.1 != big_score || item.0.rest_time() != big_time {
            loop_paths.push(item.clone());
            return false;
        }
        return true;
    });
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
