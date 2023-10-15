#![allow(unused)]
use std::{cell::RefCell, collections::HashMap, time::Instant};

use crate::utils::read_file;

mod map;
mod test;

use map::*;

type PathKey = Vec<(String, bool)>;

type PathMap = HashMap<PathKey, (usize, i32)>;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let now = Instant::now();
    let map = parse_input();
    let mut path_map: PathMap = HashMap::new();
    let first_key = vec![(String::from("AA"), false)];
    path_map.insert(first_key.clone(), (0, 30));
    let mut cur_arr = vec![first_key];

    let mut i = 0;
    loop {
        i += 1;
        for cur_path in cur_arr.iter() {
            find_path(cur_path, &map, &mut path_map, 1);
        }
        let path_arr = get_top_path(&mut path_map, &map);
        if path_arr.len() == 0 {
            break;
        }

        // println!(
        //     "index={} | key_size={} | top_path_score={:?} | top_path_time={:?}\ntop_path:{:?}",
        //     i,
        //     path_arr.len(),
        //     path_arr[0].1,
        //     path_arr[0].2,
        //     format_path(&path_arr[0].0),
        // );
        cur_arr = path_arr.into_iter().map(|item| item.0).collect();
    }

    let path_arr = get_top_path1(&path_map);

    println!(
        "cost_time={:?}\nscore={:?}|cur_time={}\npath:{:?}",
        now.elapsed(),
        &path_arr[0].1,
        &path_arr[0].2,
        format_path(&path_arr[0].0),
    );
}

fn get_top_path(
    path_map: &mut PathMap,
    map: &Switches,
) -> Vec<(Vec<(String, bool)>, usize, i32)> {
    let mut vec = path_map.iter().collect::<Vec<_>>();

    vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    let big_num = vec[0].1 .0;

    let mut arr = vec
        .into_iter()
        .filter(|item| {
            let (path, (score, time)) = item;
            let opened_arr = path
                .iter()
                .filter(|item| item.1)
                .map(|item| item.0.clone())
                .collect::<Vec<_>>();

            let mut un_open_path = map
                .list
                .iter()
                .filter(|item| {
                    if item.borrow().rate <= 0 {
                        return false;
                    }
                    if opened_arr.contains(&item.borrow().name) {
                        return false;
                    }
                    true
                })
                .map(|item| item.borrow().rate)
                .collect::<Vec<_>>();

            if un_open_path.len() == 0 {
                return false;
            }

            let mut local_big_num = calc_big_num(un_open_path, time.clone());
            local_big_num += score;
            // 没有可能 大于目前最大值的 就不用去处理了
            if local_big_num < big_num {
                // println!("remove_item:{}|{}|{}", score, time, big_num);
                // path_map.remove(*path);
                return false;
            }
            true
        })
        .filter(|item| item.1 .1 > 0)
        .map(|item| (item.0.clone(), item.1 .0, item.1 .1))
        .collect::<Vec<_>>();

    if arr.len() == 0 {
        return arr;
    }

    arr.sort_by(|a, b| b.1.cmp(&a.1));

    let big_num = arr[0].1;

    arr.into_iter().filter(|item| item.1 == big_num).collect()
}

fn get_top_path1(path_map: &PathMap) -> Vec<(Vec<(String, bool)>, usize, i32)> {
    let mut vec = path_map.iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    vec.iter()
        .map(|item| (item.0.clone(), item.1 .0, item.1 .1))
        .collect()
}

fn find_path(
    cur_path: &PathKey,
    map: &Switches,
    path_map: &mut PathMap,
    mut cur_space: i32,
) {
    if cur_space < 0 {
        return;
    }
    let cur_info = match path_map.get(cur_path) {
        Some(info) => info.clone(),
        None => return,
    };

    let last_ele = match cur_path.iter().last() {
        Some(item) => item,
        None => return,
    };

    let cur_name = last_ele.0.clone();
    let cur_value = match map.get_value(&cur_name) {
        Some(item) => item,
        None => panic!("cant find item name={}", cur_name),
    };
    let mut arr = cur_value
        .to
        .iter()
        .map(|item| {
            let mut switch = match map.get_refcell(item) {
                Some(item) => item,
                None => return None,
            };
            let bor_switch = switch.borrow();
            let rate = bor_switch.rate;
            if rate == 0 || has_opened(&bor_switch.name, cur_path) {
                return Some(vec![(switch.borrow(), 0, 0)]);
            }

            return Some(vec![
                (switch.borrow(), 0, rate),
                (switch.borrow(), rate, rate),
            ]);
        })
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .flatten()
        .collect::<Vec<_>>();
    // 将分数高的放在前面 -> 有我的get_top_path 就不需要了
    // arr.sort_by(|a, b| b.2.cmp(&a.2));

    let mut find_deep = false;
    for item in arr.iter() {
        let (switcher, rate, _) = item;
        let (cur_score, mut cur_time) = cur_info;
        let mut key = cur_path.clone();
        let opened = *rate != 0;

        let mut local_space = cur_space;
        cur_time -= 1;
        local_space -= 1;
        let mut new_core = cur_score;
        if opened {
            cur_time -= 1;
            local_space -= 1;
            new_core += *rate as usize * cur_time as usize;
        }

        if cur_time <= 0 {
            find_deep = true;
            key.push((switcher.name.clone(), opened));
            path_map.insert(key.clone(), (cur_score, 0));
            continue;
        }
        find_deep = true;
        key.push((switcher.name.clone(), opened));
        path_map.insert(key.clone(), (new_core, cur_time));
        find_path(&key, map, path_map, local_space);
    }

    if find_deep {
        path_map.remove(cur_path);
    }
}

fn calc_big_num(rate_arr: Vec<u8>, mut time: i32) -> usize {
    let mut all = 0 as usize;
    for item in rate_arr.iter() {
        time -= 2;
        if time <= 0 {
            break;
        }
        all += time as usize * *item as usize;
    }
    all
}

fn format_path(path: &PathKey) -> String {
    return path
        .iter()
        .map(|item| format!("{}-{}", item.0, item.1))
        .collect::<Vec<_>>()
        .join("|");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_result() {
        let path = test::str_to_path("AA-false|DD-true|CC-false|BB-true|AA-false|II-false|JJ-true|II-false|AA-false|DD-false|EE-false|FF-false|GG-false|HH-true|GG-false|FF-false|EE-true|DD-false|CC-true");
        test::test_path_score(path);
    }
    #[test]
    fn test_demo_path_score() {
        let path = test::str_to_path("AA-false|DD-true|CC-false|BB-true|AA-false|II-false|JJ-true|II-false|AA-false|DD-false|EE-false|FF-false|GG-false|HH-true|GG-false|FF-false|EE-true|DD-false|CC-true|DD-false|CC-false|BB-false|CC-false|BB-false|CC-false");
        test::test_path_score(path);
    }
    #[test]
    fn test_input_path_score() {
        let path = test::str_to_path("AA-false|WP-false|OB-false|XW-true|AZ-false|AD-true|GW-false|SY-false|LW-true|VF-false|RX-false|CU-true|VA-false|GH-true|PS-false|LU-false|XJ-true|LU-false|PS-false|GH-false|PS-false");
        test::test_path_score(path);
    }
}
