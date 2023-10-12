#![allow(unused)]
use std::{borrow::BorrowMut, collections::HashMap, hash::Hash};

use crate::utils::read_file;
mod map;

use map::*;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let mut map = parse_input();
    println!("{:?}", map);
    return;
    let line_index: i32 = 10;
    let mut line_empty_list = Vec::new();
    for item in map.sensor_and_bean.iter() {
        let sensor_wrap = map.get_point(item.0);
        let bean_wrap = map.get_point(item.1);
        if sensor_wrap.is_none() || bean_wrap.is_none() {
            panic!("not find points for sensor={:?} bean={:?}", item.0, item.1);
        }
        let (sensor, bean) = (sensor_wrap.unwrap(), bean_wrap.unwrap());
        let sensor_distance = sensor.distance_from_point(bean);
        let line_dis = sensor.distance_from_line(line_index);
        let distance_extra = sensor_distance - line_dis;
        if distance_extra < 0 {
            continue;
        }

        let empty_range_x =
            (sensor.x - distance_extra, sensor.x + distance_extra);
        for i in empty_range_x.0..=empty_range_x.1 {
            if sensor.y == line_index && i == sensor.x {
                continue;
            }
            if i == 2 {
                println!(
                    "sensor={:?} sensor_distance={:?}",
                    sensor, sensor_distance
                );
            }
            if line_empty_list.contains(&i) == false {
                line_empty_list.push(i);
            }
        }
    }
    line_empty_list.sort();
    line_empty_list.retain(|x: &i32| {
        let mut point = map.get_point((*x, line_index)).unwrap();
        if point.is_not_empty() {
            return false;
        }
        point.set_status(PointStatus::ConfirmEmpty);
        return true;
    });
    for (index, x) in line_empty_list.iter().enumerate() {}
    println!("{:?}", map);
    println!(
        "num={:?}\nline={:?}",
        line_empty_list.len(),
        line_empty_list,
    );
}

fn parse_input() -> Map {
    let content = read_file("day15/input.txt").unwrap();
    let regex =
        regex::Regex::new(r"Sensor at ([^\:]+): closest beacon is at ([^\:]+)")
            .unwrap();

    let info = content
        .split("\n")
        .map(|line| {
            let m = regex.captures(line).unwrap();
            let (_, match_list): (&str, [&str; 2]) = m.extract();
            (
                Point::from_str(match_list[0]),
                Point::from_str(match_list[1]),
            )
        })
        .collect::<Vec<_>>();

    let range = get_map_range(&info);
    let mut map = Map::new(range);
    map.set_sensor_and_bean(info);
    map
}

fn get_map_range(info: &Vec<(Point, Point)>) -> ((i32, i32), (i32, i32)) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let points = info
        .iter()
        .map(|item| vec![&item.0, &item.1])
        .collect::<Vec<_>>()
        .concat();

    for item in points.iter() {
        if item.x < min_x {
            min_x = item.x;
        }
        if item.x > max_x {
            max_x = item.x;
        }
        if item.y < min_y {
            min_y = item.y;
        }
        if item.y > max_y {
            max_y = item.y;
        }
    }

    return ((min_x, max_x), (min_y, max_y));
}
