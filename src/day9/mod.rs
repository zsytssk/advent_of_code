#![allow(unused)]

use crate::utils::read_file;
mod map;

use map::*;

// 如何确定T走的位置
// https://adventofcode.com/2022/day/9
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let arr = parse_input();
    let mut map: (u32, u32) = (0, 0);

    let mut cur_pos = Point::new(0, 0);
    let mut move_points = Vec::new();

    for item in &arr {
        adjust_map(&cur_pos, item, &mut map);
        let points = cur_pos.get_move_points(&item);
        cur_pos = cur_pos.move_to(&item);
        for point in points {
            if move_points.contains(&point) == false {
                move_points.push(point);
            }
        }
    }
    println!("{:?} {:?} {:?}", map, move_points, cur_pos);
}

fn parse2() {}

fn adjust_map(pos: &Point, mo: &Move, map: &mut (u32, u32)) {
    let target_pos = pos.move_to(mo);
    if target_pos.0 > map.0 as i32 {
        map.0 = target_pos.0 as u32
    }
    if target_pos.1 > map.1 as i32 {
        map.1 = target_pos.1 as u32
    }
}

fn parse_input() -> Vec<Move> {
    let content = read_file("day9/demo.txt").unwrap();
    let arr = content
        .split("\n")
        .map(|line| {
            let arr = line.split(" ").collect::<Vec<&str>>();
            let dir = Dir::from_str(arr[0]);
            let space = arr[1].parse::<i32>().unwrap();

            (dir, space)
        })
        .collect::<Vec<_>>();

    arr
}
