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
    let mut complete_path_map: PathMap = HashMap::new();
    let mut loop_path_map: PathMap = HashMap::new();
    let first_key = vec![(String::from("AA"), false)];
    loop_path_map.insert(first_key.clone(), (0, 30));
    let mut cur_arr = vec![first_key];

    let mut i = 0;
    loop {
        i += 1;
        for cur_path in cur_arr.iter() {
            find_path(cur_path, &map, &mut loop_path_map, 1);
        }
        let path_arr =
            calc_top_path(&mut loop_path_map, &mut complete_path_map, &map);
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

    let path_arr = get_top_path(&complete_path_map);
    println!(
        "cost_time={:?}\nscore={:?}|cur_time={}\npath:{:?}",
        now.elapsed(),
        &path_arr[0].1,
        &path_arr[0].2,
        format_path(&path_arr[0].0),
    );
}

fn calc_top_path(
    loop_path_map: &mut PathMap,
    complete_path_map: &mut PathMap,
    map: &Switches,
) -> Vec<(Vec<(String, bool)>, usize, i32)> {
    let mut vec = complete_path_map.iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
    let big_num = match vec.first() {
        Some(t) => t.1 .0,
        None => 0,
    };
    complete_path_map.retain(|path, item| item.0 == big_num);

    let mut arr = Vec::new();
    loop_path_map.retain(|path, item| {
        let (score, time) = item;
        let opened_arr = path
            .iter()
            .filter(|item| item.1)
            .map(|item| item.0.clone())
            .collect::<Vec<_>>();

        if *time <= 0 {
            if *score > big_num {
                complete_path_map.insert(path.clone(), item.clone());
            }
            return false;
        }

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
            if *score > big_num {
                complete_path_map.insert(path.clone(), item.clone());
            }
            return false;
        }

        let mut local_big_num = calc_big_num(un_open_path, time.clone());
        local_big_num += *score;
        // 没有可能 大于目前最大值的 就不用去处理了
        if local_big_num < big_num {
            // println!("remove_item:{}|{}|{}", score, time, big_num);
            return false;
        }
        arr.push((path.clone(), item.0, item.1));
        return true;
    });

    if arr.len() == 0 {
        return arr;
    }

    arr.sort_by(|a, b| {
        let rate_cmp = b.1.cmp(&a.1);
        if rate_cmp != std::cmp::Ordering::Equal {
            return rate_cmp;
        }
        b.2.cmp(&a.2)
    });

    let big_num = arr[0].1;
    let big_time = arr[0].2;
    println!(
        "arr_size:{}|big_num:{}|big_time:{}",
        arr.len(),
        big_num,
        big_time
    );

    arr.into_iter()
        .filter(|item| item.1 == big_num && item.2 == big_time)
        .collect()
}

fn get_top_path(path_map: &PathMap) -> Vec<(Vec<(String, bool)>, usize, i32)> {
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

fn calc_big_num(mut rate_arr: Vec<u8>, mut time: i32) -> usize {
    let mut all = 0 as usize;
    rate_arr.sort_by(|a, b| b.cmp(a));
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
    let content = read_file("day16/input.txt").unwrap();

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
        let path = test::str_to_path("AA-false|DD-true|AA-false|BB-true|AA-false|II-false|JJ-true|II-false|AA-false|DD-false|EE-true|FF-false|GG-false|HH-true|GG-false|FF-false|EE-false|DD-false|CC-true");
        test::test_path_score(path);
    }
    #[test]
    fn test_input_path_score() {
        let path = test::str_to_path("AA-false|WP-false|OB-false|XW-true|AZ-false|AD-true|GW-false|SY-false|LW-true|VF-false|RX-false|CU-true|VA-false|GH-true|PS-false|LU-false|XJ-true|LU-false|PS-false|GH-false|PS-false");
        test::test_path_score(path);
    }
}
