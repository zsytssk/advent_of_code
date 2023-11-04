#![allow(unused)]

use crate::utils::read_file;

// https://adventofcode.com/2022/day/8
pub fn parse() {
  // parse1();
  parse2();
}

fn parse1() {
  let arr = parse_input();
  let mut can_see_arr = vec![];
  for (i, line) in arr.iter().enumerate() {
    for (j, item) in line.iter().enumerate() {
      let column = arr.iter().map(|arr| arr[j]).collect::<Vec<u32>>();
      let cur = arr[i][j];
      if can_see_from_outer(j, line) || can_see_from_outer(i, &column) {
        can_see_arr.push((cur, i, j));
      }
    }
  }

  println!("{:?}", can_see_arr.len());
}

fn parse2() {
  let arr = parse_input();
  let mut can_see_arr = vec![];
  for (i, line) in arr.iter().enumerate() {
    for (j, item) in line.iter().enumerate() {
      let column = arr.iter().map(|arr| arr[j]).collect::<Vec<u32>>();
      let cur = arr[i][j];
      let (x1, x2) = get_view_distance(j, line);
      let (y1, y2) = get_view_distance(i, &column);
      let val = (cur, i, j, x1 * x2 * y1 * y2);
      if i == 3 && j == 2 {
        println!("{:?}", val);
      }
      can_see_arr.push(val);
    }
  }

  let max_num = can_see_arr.iter().max_by(|a, b| a.3.cmp(&b.3)).unwrap();

  println!("{:?}", max_num);
}

fn can_see_from_outer(index: usize, arr: &Vec<u32>) -> bool {
  if index == 0 || index == arr.len() - 1 {
    return true;
  }

  let cur = arr[index];

  let mut i = 0;
  while i < arr.len() {
    if i < index {
      if arr[i] >= cur {
        i = index + 1;
        continue;
      }
      if i == index - 1 {
        return true;
      }
      i += 1;
      continue;
    }

    if i > index {
      if arr[i] >= cur {
        break;
      }
      if i == arr.len() - 1 {
        return true;
      }
    }
    i += 1;
  }

  false
}
fn get_view_distance(index: usize, arr: &Vec<u32>) -> (i32, i32) {
  if index == 0 || index == arr.len() - 1 {
    return (0, 0);
  }

  let cur = arr[index];
  let mut num_left = 0;
  let mut num_right = 0;
  let mut i = index - 1;

  while i < arr.len() {
    if i < index {
      num_left += 1;
      if i == 0 || arr[i] >= cur {
        i = index + 1;
        continue;
      }
      i -= 1;
      continue;
    }

    if i > index {
      num_right += 1;
      if arr[i] >= cur {
        break;
      }
      i += 1;
    }
  }

  (num_left, num_right)
}

fn parse_input() -> Vec<Vec<u32>> {
  let content = read_file("day8/input.txt").unwrap();

  let arr = content
    .split("\n")
    .map(|line| {
      line
        .split("")
        .filter(|&s| s != "")
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
    })
    .collect::<Vec<_>>();

  arr
}
