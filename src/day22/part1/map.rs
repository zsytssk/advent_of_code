use std::{cell::RefCell, fmt};

use regex::Regex;

use super::super::utils::split_keep;

pub struct Map {
  pub lines: Vec<Vec<Point>>,
  x_size: usize,
}

impl Map {
  pub fn from_str(map: &str) -> Self {
    let lines_str = map.split("\n").collect::<Vec<_>>();

    let x_size = lines_str.iter().map(|item| item.len()).max().unwrap();

    let mut lines = vec![];
    for (y, line_str) in lines_str.iter().enumerate() {
      let mut line = vec![];
      for x in 0..x_size {
        let letter = match x < line_str.len() {
          true => line_str.chars().nth(x).unwrap(),
          false => ' ',
        };
        let point = Point::new(x + 1, y + 1, letter);
        line.push(point);
      }
      lines.push(line);
    }

    Map { lines, x_size }
  }
  pub fn get_start(&self) -> &Point {
    match self.lines.get(0) {
      Some(line) => {
        for p in line.iter() {
          if p.type_name == PointType::Tile {
            return p;
          }
        }
      }
      None => panic!("no start"),
    };
    panic!("no start");
  }
  pub fn get_p(&self, x: usize, y: usize) -> &Point {
    match self.lines.get(y - 1) {
      Some(line) => line.get(x - 1).unwrap(),
      None => panic!("no find point"),
    }
  }
  pub fn get_mut_p(&mut self, x: usize, y: usize) -> &mut Point {
    match self.lines.get_mut(y - 1) {
      Some(line) => line.get_mut(x - 1).unwrap(),
      None => panic!("no find point"),
    }
  }
  pub fn peek_next(&self, x: usize, y: usize) -> &Point {
    match self.lines.get(y) {
      Some(line) => line.get(x).unwrap(),
      None => panic!("no find point"),
    }
  }
  /** 获得当前[块] 的反方向的点 */
  pub fn get_opposite_p<'a>(
    &'a self,
    cur_p: &'a Point,
    dir: &Direction,
  ) -> Option<&'a Point> {
    let x_size = &self.x_size;
    let change_pos = match dir {
      Direction::Right => (-1, 0),
      Direction::Left => (1, 0),
      Direction::Up => (0, 1),
      Direction::Down => (0, -1),
    };

    let mut move_p = cur_p;
    loop {
      let new_pos = (
        (move_p.x as i32 + change_pos.0) as i32,
        (move_p.y as i32 + change_pos.1) as i32,
      );

      // 不存在超出范围的情况
      if new_pos.0 < 1
        || new_pos.0 > *x_size as i32
        || new_pos.1 < 1
        || new_pos.1 > self.lines.len() as i32
      {
        // println!(
        //   "get_opposite_p out of range | cur_p={:?} | new_pos={:?}",
        //   cur_p, new_pos
        // );
        break;
      }

      let p = self.get_p(new_pos.0 as usize, new_pos.1 as usize);
      if p.type_name == PointType::Empty {
        break;
      }
      move_p = p;
    }

    return Some(move_p);
  }
  pub fn get_next_pos<'a>(
    &'a self,
    cur_pos: &'a Point,
    path_item: &PathItem,
    path_arr: &mut Vec<(usize, usize, Direction)>,
  ) -> &'a Point {
    let dir = &path_item.dir;
    let num = path_item.num;
    let x_size = &self.x_size;

    let mut move_pos = cur_pos;
    path_arr.push(move_pos.get_pos(dir));

    for _ in 0..num {
      let mut x = move_pos.x as i32;
      let mut y = move_pos.y as i32;

      match dir {
        Direction::Right => {
          x += 1;
        }
        Direction::Left => {
          x -= 1;
        }
        Direction::Down => {
          y += 1;
        }
        Direction::Up => {
          y -= 1;
        }
      }

      // println!("{:?} | {},{}", move_pos, x, y);
      // 超出范围
      if x < 1 || x > *x_size as i32 || y < 1 || y > (self.lines.len()) as i32 {
        match self.get_opposite_p(move_pos, &dir) {
          Some(p) => {
            if p.type_name == PointType::Wall {
              // println!("break1:>");
              break;
            }
            move_pos = p;
            path_arr.push(move_pos.get_pos(&dir));
          }
          None => unreachable!(),
        }
        continue;
      }

      let p = self.get_p(x as usize, y as usize);
      if p.type_name == PointType::Wall {
        // println!("break2:>");
        break;
      }
      if p.type_name == PointType::Empty {
        match self.get_opposite_p(move_pos, &dir) {
          Some(p) => {
            if p.type_name == PointType::Wall {
              // println!("break3:>");
              break;
            }
            move_pos = p;
            path_arr.push(move_pos.get_pos(&dir));
            continue;
          }
          None => unreachable!(),
        }
      }

      move_pos = p;
      path_arr.push(move_pos.get_pos(&dir));
    }

    move_pos
  }
}

impl fmt::Debug for Map {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut s = String::from("");
    for (index, line) in self.lines.iter().enumerate() {
      for p in line.iter() {
        s += p.as_str();
      }
      if index != self.lines.len() - 1 {
        s += "\n";
      }
    }
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Debug)]
pub enum PointType {
  Empty,
  Tile,
  Wall,
}

pub struct Point {
  pub x: usize,
  pub y: usize,
  pub type_name: PointType,
  pub move_dir: Option<Direction>,
}

impl Point {
  pub fn new(x: usize, y: usize, letter: char) -> Self {
    let type_name = match letter {
      ' ' => PointType::Empty,
      '.' => PointType::Tile,
      '#' => PointType::Wall,
      _ => unreachable!(),
    };

    Point {
      x,
      y,
      type_name,
      move_dir: None,
    }
  }
  pub fn get_pos(&self, dir: &Direction) -> (usize, usize, Direction) {
    (self.x, self.y, dir.clone())
  }
  pub fn set_move_dir(&mut self, move_dir: Direction) {
    self.move_dir = Some(move_dir);
  }
  pub fn as_str(&self) -> &str {
    let move_dir_wrap = self.move_dir.clone();
    if move_dir_wrap.is_some() {
      let move_dir = move_dir_wrap.unwrap();
      match move_dir {
        Direction::Right => {
          return ">";
        }
        Direction::Left => {
          return "<";
        }
        Direction::Down => {
          return "v";
        }
        Direction::Up => {
          return "^";
        }
      }
    }
    match self.type_name {
      PointType::Empty => " ",
      PointType::Tile => ".",
      PointType::Wall => "#",
    }
  }
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

#[derive(Debug)]
pub struct PathItem {
  pub dir: Direction,
  pub num: usize,
}

#[derive(Debug)]
pub struct Path {
  pub list: Vec<PathItem>,
}

impl Path {
  pub fn from_str(text: &str) -> Self {
    let re = Regex::new(r"R|L").unwrap();
    let arr = split_keep(re, text);
    let mut cur_dir = Direction::Right;
    let mut list = vec![];
    for (index, item) in arr.iter().enumerate() {
      let mut is_last_only_dir = false;
      match item.as_str() {
        "R" => {
          cur_dir = cur_dir.change_dir("R");
          if index == arr.len() - 1 {
            is_last_only_dir = true;
          }
        }
        "L" => {
          cur_dir = cur_dir.change_dir("L");
          if index == arr.len() - 1 {
            is_last_only_dir = true;
          }
        }
        _ => {
          let num = item.parse::<usize>().unwrap();
          list.push(PathItem {
            dir: cur_dir.clone(),
            num,
          });
        }
      };

      if is_last_only_dir {
        list.push(PathItem {
          dir: cur_dir.clone(),
          num: 0,
        });
      }
    }

    Path { list }
  }
  pub fn get_item(&self, index: usize) -> &PathItem {
    match self.list.get(index) {
      Some(item) => item,
      None => panic!("no item"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
  Right = 0,
  Down = 1,
  Left = 2,
  Up = 3,
}

impl Direction {
  pub fn change_dir(self, dir_str: &str) -> Self {
    let mut num = self as i8;

    match dir_str {
      "R" => num += 1,
      "L" => num -= 1,
      _ => unreachable!(),
    };

    if num >= 4 {
      num -= 4
    } else if num < 0 {
      num += 4
    }

    match num {
      0 => Direction::Right,
      1 => Direction::Down,
      2 => Direction::Left,
      3 => Direction::Up,
      _ => unreachable!(),
    }
  }
}
