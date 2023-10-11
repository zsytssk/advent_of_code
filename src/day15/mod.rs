#![allow(unused)]
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
    let mut target_line = &mut map.points[10];
    for item in map.sensor_and_bean.iter() {
        let sensor = map.get_point(item.0);
        let bean = map.get_point(item.1);
        if sensor.is_none() || bean.is_none() {
            panic!("not find points for sensor={:?} bean={:?}", item.0, item.1);
        }
        println!("{:?} {:?}", sensor, bean);
    }
}

fn parse_input() -> Map {
    let content = read_file("day15/demo.txt").unwrap();
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
