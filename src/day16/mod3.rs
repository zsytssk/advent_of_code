#![allow(unused)]
use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    cmp::Ordering,
    collections::HashMap,
    fmt,
    time::Instant,
};

use crate::utils::read_file;

mod switch;
mod test;

use switch::*;

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
        if cur_arr.len() == 0 {
            return;
        }
        i += 1;
        for cur_path in cur_arr.iter() {
            find_path(cur_path, &map, &mut path_map, 3);
        }
        let path_arr = get_top_path(&mut path_map, &map);
        println!(
            "index={}| key_size={} | top_path_score={:?}\ntop_path:{:?}",
            i,
            path_arr.len(),
            path_arr[0].1,
            format_path(&path_arr[0].0),
        );
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

    vec.iter()
        .filter(|item| {
            let (path, (score, time)) = item;
            let opened_arr = path
                .iter()
                .filter(|item| item.1)
                .map(|item| item.0.clone())
                .collect::<Vec<_>>();

            let mut all_path = map
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

            let mut big_num = calc_big_num(all_path, time.clone());
            big_num += score;
            // 没有可能 大于目前最大值的 就不用去处理了
            if big_num < big_num {
                println!("remove_item:{}|{}|{}", score, time, big_num);
                return false;
            }
            true
        })
        .filter(|item| item.1 .1 > 0)
        .map(|item| (item.0.clone(), item.1 .0, item.1 .1))
        .collect()
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

    if cur_info.1 <= 0 {
        return;
    }

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
                let has_path = has_pass_path(
                    [last_ele.clone(), (bor_switch.name.clone(), false)],
                    cur_path,
                );

                return Some(vec![(switch.borrow(), 0, 0, has_path)]);
            }
            let opened_has_path = has_pass_path(
                [last_ele.clone(), (bor_switch.name.clone(), true)],
                cur_path,
            );
            let has_path = has_pass_path(
                [last_ele.clone(), (bor_switch.name.clone(), false)],
                cur_path,
            );

            return Some(vec![
                (switch.borrow(), 0, rate, has_path),
                (switch.borrow(), rate, rate, opened_has_path),
            ]);
        })
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .flatten()
        .filter(|item| !item.3)
        .collect::<Vec<_>>();

    arr.sort_by(|a, b| b.2.cmp(&a.2));

    let mut find_deep = false;
    for item in arr.iter() {
        let (switcher, rate, _, _) = item;
        let (mut cur_score, mut cur_time) = cur_info;
        let mut key = cur_path.clone();
        let opened = *rate != 0;

        let mut local_space = cur_space;
        cur_time -= 1;
        local_space -= 1;
        if opened {
            cur_time -= 1;
            local_space -= 1;
            cur_score += *rate as usize * cur_time as usize;
        }

        if cur_time < 0 {
            continue;
        }
        find_deep = true;
        key.push((switcher.name.clone(), opened));
        path_map.insert(key.clone(), (cur_score, cur_time));
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

fn has_pass_path(part_path: [(String, bool); 2], whole_path: &PathKey) -> bool {
    for (index, item) in whole_path.iter().enumerate() {
        if item.0 != part_path[0].0 || item.1 != part_path[0].1 {
            continue;
        }
        if index == whole_path.len() - 1 {
            continue;
        }
        let next_item = &whole_path[index + 1];
        if next_item.0 == part_path[1].0 && next_item.1 == part_path[1].1 {
            return true;
        }
    }

    false
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
    fn test_path_score() {
        let path = test::str_to_path("AA-false|DD-true|AA-false|II-false|JJ-true|II-false|AA-false|BB-true|CC-true|DD-false|EE-true|FF-false|GG-false|HH-true");
        test::test_path_score(path);
    }
    #[test]
    fn test_pass_path() {
        test::test_pass_path();
    }
}
