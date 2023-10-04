#![allow(unused)]

use std::vec;

use crate::utils::read_file;
mod map;

use map::*;

// https://adventofcode.com/2022/day/9
pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let arr = parse_input();
    // let mut map: (u32, u32) = (0, 0);

    let mut head_pos = Point::new(0, 0);
    let mut tail_pos = Point::new(0, 0);
    let mut tail_move_points = vec![tail_pos];

    for item in &arr {
        // adjust_map(&head_pos, item, &mut map);
        let head_points = head_pos.get_move_points(&item);
        for hp in head_points.iter() {
            match get_tail_next(&tail_pos, hp) {
                Some(pt) => {
                    // println!("{:?} {:?} {:?}", &hp, &tail_pos, &pt);
                    if tail_move_points.contains(&pt) == false {
                        tail_move_points.push(pt);
                    }
                    tail_pos = pt;
                }
                _ => continue,
            }
        }
        head_pos = head_points[head_points.len() - 1].clone();
    }
    println!("tail_move_points={:?}", tail_move_points.len());
}

fn parse2() {
    let arr = parse_input();

    let mut head_pos = Point::new(0, 0);
    let mut rest_pos_arr: Vec<Point> = Vec::new();
    for i in 0..9 {
        rest_pos_arr.push(Point::new(0, 0))
    }
    let mut tail_move_points = vec![rest_pos_arr[8]];

    for item in &arr {
        let head_points = head_pos.get_move_points(&item);
        for hp in head_points.iter() {
            let mut cur_hp = hp.clone();
            for (index, item_p) in rest_pos_arr.iter_mut().enumerate() {
                match get_tail_next(&item_p, &cur_hp) {
                    Some(pt) => {
                        cur_hp = pt.clone();
                        *item_p = pt.clone();
                    }
                    _ => {
                        cur_hp = item_p.clone();
                    }
                }
            }
            if tail_move_points.contains(&cur_hp) == false {
                tail_move_points.push(cur_hp);
            }
        }
        head_pos = head_points[head_points.len() - 1].clone();
    }
    println!("tail_move_points={:?}", tail_move_points.len());
}

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
    let content = read_file("day9/input.txt").unwrap();
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
