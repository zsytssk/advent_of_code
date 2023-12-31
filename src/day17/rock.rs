use std::{fmt, vec};

use super::utils::create_dots;

pub enum Dir {
  Left,
  Right,
  Down,
}

impl Dir {
  pub fn from_str(s: &char) -> Self {
    match s {
      '<' => Dir::Left,
      '>' => Dir::Right,
      _ => Dir::Down,
    }
  }
}

#[derive(Debug)]
pub struct Chamber {
  pub rock_list: Vec<Vec<Rock>>,
  pub width: usize,
  pub height: usize,
  pub split_num: usize,
  pub top_height: usize,
}
impl Chamber {
  pub fn new(width: usize, height: usize) -> Self {
    Chamber {
      rock_list: Vec::new(),
      width,
      height,
      split_num: 10,
      top_height: 0,
    }
  }
  pub fn get_top_height(&self) -> usize {
    return self.top_height;
  }
  pub fn adjust_rock(&mut self, rock: &mut Rock) {
    let range = rock.get_inner_range();
    let top_height = self.top_height;
    let new_height = top_height + range.1 + 3;

    rock.set_pos(2, new_height - range.1);

    self.height = new_height;
  }
  pub fn add_rock(&mut self, mut rock: Rock) {
    let big_y = rock.y + rock.get_inner_range().1;
    if big_y > self.top_height {
      self.top_height = big_y;
    }

    let step_index = big_y / self.split_num;
    match self.rock_list.get_mut(step_index) {
      None => {
        self.rock_list.push(vec![rock]);
      }
      Some(t) => t.push(rock),
    }
  }
  pub fn move_rock_to(&mut self, rock: &mut Rock, dir: Dir) -> bool {
    let mut x = rock.x as i64;
    let mut y = rock.y as i64;
    let range = rock.get_inner_range();

    match dir {
      Dir::Left => {
        x -= 1;
      }
      Dir::Right => {
        x += 1;
      }
      Dir::Down => {
        y -= 1;
      }
    }
    if x < 0 || y < 0 {
      return false;
    }

    if x + range.0 as i64 > self.width as i64 {
      return false;
    }

    let rel_rocks = self.get_rel_rock(&rock);

    for dot in rock.dots.iter() {
      let rx = dot.x + x as usize;
      let ry = dot.y + y as usize;
      for item in rel_rocks.iter() {
        if item.contain_dots(&Dot { x: rx, y: ry }) {
          return false;
        }
      }
    }

    rock.x = x as usize;
    rock.y = y as usize;

    true
  }
  /** 获取周围的rock */
  pub fn get_rel_rock(&self, rock: &Rock) -> Vec<&Rock> {
    let y = rock.y;
    let range = rock.get_inner_range();
    let big_y = y + range.1;

    let list_index = big_y / self.split_num;

    let mut list = vec![];
    let mut index_arr = vec![list_index, list_index + 1];
    if list_index > 0 {
      index_arr.insert(0, list_index - 1);
    }

    for index in index_arr {
      match self.rock_list.get(index) {
        None => {}
        Some(rock_list) => list.extend(rock_list),
      };
    }

    list
  }

  pub fn get_fmt_str(&self) -> String {
    let width = self.width;
    let height = self.height;

    let mut all = String::from("");
    // for y in 0..height {
    //     let mut line = String::from("");
    //     for x in 0..width {
    //         if self.contain_dots(&Dot { x, y }) {
    //             line = format!("{}#", line);
    //         } else {
    //             line = format!("{}.", line);
    //         }
    //     }
    //     all = format!("{}\n{}", line, all);
    // }

    all
  }
}

#[derive(Clone)]
pub enum RockShape {
  /** 四个在一行 长条形 `-` */
  LineFour,
  /** 交叉 5个 `+` */
  CrossLine,
  /** l形 `l` */
  LShape,
  /** 竖行 `|` */
  Vertical,
  /** 正方形 `■` */
  Square,
}

pub struct Rock {
  dots: Vec<Dot>,
  shape: RockShape,
  x: usize,
  y: usize,
}

impl Rock {
  pub fn new(shape: RockShape, x: usize, y: usize) -> Self {
    Rock {
      dots: create_dots(&shape),
      shape,
      x,
      y,
    }
  }
  pub fn contain_dots(&self, dot: &Dot) -> bool {
    let x = (dot.x as i64) - (self.x as i64);
    let y = (dot.y as i64) - (self.y as i64);
    let range = self.get_inner_range();
    if x < 0 || y < 0 {
      return false;
    }

    if x as usize > range.0 || y as usize > range.1 {
      return false;
    }
    self.dots.contains(&Dot {
      x: x as usize,
      y: y as usize,
    })
  }
  pub fn set_pos(&mut self, x: usize, y: usize) {
    self.x = x;
    self.y = y;
  }
  pub fn get_range(&self) -> ((usize, usize), (usize, usize)) {
    let inner_rage = self.get_inner_range();
    (
      (self.x, self.x + inner_rage.0),
      (self.y, self.y + inner_rage.1),
    )
  }
  pub fn get_inner_range(&self) -> (usize, usize) {
    let mut big_x = 0;
    let mut big_y = 0;
    for dot in self.dots.iter() {
      if dot.x > big_x {
        big_x = dot.x
      }
      if dot.y > big_y {
        big_y = dot.y
      }
    }

    (big_x + 1, big_y + 1)
  }
}

impl fmt::Debug for Rock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut big_x = 0;
    let mut big_y = 0;
    for dot in self.dots.iter() {
      if dot.x > big_x {
        big_x = dot.x
      }
      if dot.y > big_y {
        big_y = dot.y
      }
    }

    let mut all = String::from("");
    for y in 0..=big_y {
      let mut line = String::from("");
      for x in 0..=big_x {
        if self.dots.contains(&Dot { x, y }) {
          line = format!("{}#", line);
        } else {
          line = format!("{}.", line);
        }
      }
      all = format!("{}\n{}", line, all);
    }

    write!(f, "{}", all)
  }
}

#[derive(Debug, PartialEq)]
pub struct Dot {
  pub x: usize,
  pub y: usize,
}

impl Dot {
  pub fn new(x: usize, y: usize) -> Self {
    Dot { x, y }
  }
}
