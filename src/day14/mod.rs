#![allow(unused)]
use crate::utils::read_file;
mod map;

use map::*;

// 要先画一个map
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let mut map = parse_input();
    'outer: loop {
        let mut sand_pos = (500, 0);
        let start_pos = map.get_point(&sand_pos);
        if start_pos.unwrap().can_move_to() == false {
            break;
        }
        loop {
            let next_points = [
                // 向下
                (sand_pos.0, sand_pos.1 + 1),
                // 向左
                (sand_pos.0 - 1, sand_pos.1 + 1),
                // 向右
                (sand_pos.0 + 1, sand_pos.1 + 1),
            ];
            let mut next_point: Option<(usize, usize)> = None;
            for np in next_points {
                let point_wrap = map.get_point(&np);
                if point_wrap.is_none() {
                    break 'outer;
                }
                let new_pos = point_wrap.unwrap();
                if new_pos.can_move_to() {
                    // 向左
                    next_point = Some((np.0, np.1));
                    break;
                }
            }

            if next_point.is_none() {
                let point = map.get_mut_point(&sand_pos).unwrap();
                point.set_status(PointStatus::Sand);
                break;
            } else {
                sand_pos = next_point.unwrap();
            }
        }
        println!("map:\n{:?}", map);
    }
    let sands = map.get_sand_points();
    println!("sand num = {}; map:\n{:?}", sands.len(), map);
}

fn parse2() {
    let mut map = parse_input();
    map.add_bottom(200);

    'outer: loop {
        let mut sand_pos = (500, 0);
        let start_pos = map.get_point(&sand_pos);

        if start_pos.unwrap().can_move_to() == false {
            break;
        }
        loop {
            let next_points = [
                // 向下
                (sand_pos.0, sand_pos.1 + 1),
                // 向左
                (sand_pos.0 - 1, sand_pos.1 + 1),
                // 向右
                (sand_pos.0 + 1, sand_pos.1 + 1),
            ];
            let mut next_point: Option<(usize, usize)> = None;
            for np in next_points {
                let point_wrap = map.get_point(&np);
                if point_wrap.is_none() {
                    break 'outer;
                }
                let new_pos = point_wrap.unwrap();
                if new_pos.can_move_to() {
                    // 向左
                    next_point = Some((np.0, np.1));
                    break;
                }
            }
            if next_point.is_none() {
                let point = map.get_mut_point(&sand_pos).unwrap();
                point.set_status(PointStatus::Sand);
                break;
            } else {
                sand_pos = next_point.unwrap();
            }
        }
        // println!("map:\n{:?}", map);
    }
    let sands = map.get_sand_points();
    println!("sand num = {}; map:\n{:?}", sands.len(), map);
    // println!("sand num = {}", sands.len());
}

fn parse_input() -> Map {
    let content = read_file("day14/demo.txt").unwrap();

    let paths = content
        .split("\n")
        .map(|line| {
            let points =
                line.split(" -> ").map(Point::from_str).collect::<Vec<_>>();
            Path::new(points)
        })
        .collect::<Vec<_>>();

    Map::new(paths)
}
