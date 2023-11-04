#![allow(unused)]
use std::{collections::HashMap, time::Instant};

use crate::{day18::cube::SurfaceStatus, utils::read_file};

use self::cube::{Cube, Surface};

mod cube;

pub fn parse() {
  // parse1();
  parse2();
}

fn parse1() {
  let now = Instant::now();
  let list = parse_input();
  let mut overlap_num = 0;
  for i in 0..(list.len() - 1) {
    for j in i + 1..list.len() {
      let cube1 = &list[i];
      let cube2 = &list[j];
      if cube1.is_overlap(cube2) {
        overlap_num += 1;
      }
    }
  }
  let res = 6 * list.len() - 2 * overlap_num;
  println!(
    "time={:?}|res={:?}|overlap_num={:?}",
    now.elapsed(),
    res,
    overlap_num
  );
}

fn parse2() {
  let now = Instant::now();
  let list = parse_input();
  let mut surface_map = HashMap::new();

  for cube in list.iter() {
    let mut cur_cube = cube;
    let surfaces = &cur_cube.surfaces;
    for surface in surfaces {
      if surface_map.contains_key(&(cube, surface)) {
        continue;
      }
      let is_overlap = surface_is_overlap(cube, surface, &list);
      if is_overlap {
        surface_map.insert((cube, surface), SurfaceStatus::Overlap);
      }
    }
  }
  for cube in list.iter() {
    let mut cur_cube = cube;
    let surfaces = &cur_cube.surfaces;
    for surface in surfaces {
      if surface_map.contains_key(&(cube, surface)) {
        continue;
      }
      loop_surface(cube, surface, &list, &mut surface_map);
    }
    // println!(
    //     "time={:?}|surface_map.len={:?}",
    //     now.elapsed(),
    //     surface_map.len()
    // );
  }

  surface_map.retain(|key, value| match value {
    SurfaceStatus::Outer => true,
    _ => false,
  });

  println!(
    "time={:?}|surface_map={:?}",
    now.elapsed(),
    surface_map.len()
  );
}

fn loop_surface<'a>(
  cube: &'a Cube,
  surface: &'a Surface,
  list: &'a Vec<Cube>,
  surface_map: &mut HashMap<(&'a Cube, &'a Surface), SurfaceStatus>,
) {
  let mut save_arr: Vec<(&Cube, &Surface)> = vec![];
  let mut loop_arr = vec![(cube, surface)];
  let mut next_arr = vec![];
  let mut surface_status = SurfaceStatus::Outer;

  'outer_loop: loop {
    let mut cur_loop_arr = vec![];
    for cur_item in loop_arr.iter() {
      // 如果已经找到了，直接返回
      match surface_map.get(&(cur_item.0, cur_item.1)) {
        Some(item_status) => {
          println!("test:>3");
          surface_status = item_status.clone();
          break 'outer_loop;
        }
        None => {}
      }

      save_arr.push((cur_item.0, cur_item.1));

      next_arr = get_cross_surfaces(cur_item.0, cur_item.1, list);
      next_arr.retain(|item| match surface_map.get(item) {
        None => true,
        Some(status) => status != &SurfaceStatus::Overlap,
      });

      /** 没有找齐四个，就是不完整的，直接 */
      println!("test:>1:>next_arr={:?}", next_arr.len());
      if next_arr.len() < 4 {
        break 'outer_loop;
      }
      let mut all_contained = true;
      for item in next_arr.iter() {
        if save_arr.contains(&item) == false {
          all_contained = false
        }
      }

      if all_contained {
        println!("test:>2");
        surface_status = SurfaceStatus::Inner;
        break 'outer_loop;
      }
      cur_loop_arr.extend(next_arr);
      next_arr = vec![];
    }

    loop_arr = cur_loop_arr;
  }

  save_arr.extend(loop_arr);
  save_arr.extend(next_arr);

  for item in save_arr {
    surface_map.insert(item, surface_status.clone());
  }
}

fn get_cross_surfaces<'a>(
  cube: &Cube,
  surface: &Surface,
  list: &'a Vec<Cube>,
) -> Vec<(&'a Cube, &'a Surface)> {
  let mut arr = vec![];
  for item in list {
    if item == cube {
      continue;
    }
    let surfaces = &item.surfaces;
    for surface_item in surfaces {
      if surface.is_cross(surface_item) {
        arr.push((item, surface_item));
      }
    }
  }

  return arr;
}
fn surface_is_overlap<'a>(
  cube: &Cube,
  surface: &Surface,
  list: &'a Vec<Cube>,
) -> bool {
  for item in list {
    if item == cube {
      continue;
    }
    let surfaces = &item.surfaces;
    for surface_item in surfaces {
      if surface.is_overlap(surface_item) {
        return true;
      }
    }
  }

  return false;
}

fn parse_input() -> Vec<Cube> {
  let content = read_file("day18/demo.txt").unwrap();

  content
    .split("\n")
    .map(|item| {
      let cor = item
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
      Cube::new(cor[0], cor[1], cor[2])
    })
    .collect::<Vec<_>>()
}
