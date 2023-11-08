use std::{
  cell::{Ref, RefCell},
  fmt,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MoveKey {
  list: Vec<(i32, i32)>,
}

impl MoveKey {
  pub fn new(list: Vec<(i32, i32)>) -> MoveKey {
    MoveKey { list }
  }
  pub fn add_pos(&mut self, pos: (i32, i32)) {
    self.list.push(pos);
  }
  pub fn get_last_pos(&self) -> &(i32, i32) {
    &self.list[self.list.len() - 1]
  }
  pub fn is_end(&self, map: &Map) -> bool {
    let end = map.get_end();
    self.list.contains(&end)
  }
  pub fn get_num(&self) -> usize {
    self.list.len()
  }
  pub fn get_quick_steps(&self, map: &Map) -> usize {
    let move_step = self.list.len();
    let last_key = &self.list[self.list.len() - 1];
    let end = map.get_end();

    let dis_end =
      last_key.0.abs_diff(end.0) as usize + last_key.1.abs_diff(end.1) as usize;

    dis_end + move_step
  }
}

pub struct Map {
  pub points: Vec<Vec<RefCell<Point>>>,
}

impl Map {
  pub fn from_str(s: &str) -> Map {
    let points = s
      .split("\n")
      .enumerate()
      .map(|(y, line)| {
        line
          .split("")
          .filter(|&item| item != "")
          .enumerate()
          .map(|(x, item)| {
            let p = Point::new(x as i32, y as i32, item);
            RefCell::new(p)
          })
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();
    Map { points }
  }
  pub fn is_outer_range(&self, pos: &(i32, i32)) -> bool {
    let (x, y) = pos;
    if x < &0
      || y < &0
      || x >= &(self.points[0].len() as i32)
      || y >= &(self.points.len() as i32)
    {
      return true;
    }
    return false;
  }
  pub fn get_p(&self, pos: &(i32, i32)) -> &RefCell<Point> {
    let (x, y) = pos;
    &self.points[*y as usize][*x as usize]
  }
  pub fn get_start(&self) -> (i32, i32) {
    (1, 0)
  }
  pub fn get_end(&self) -> (i32, i32) {
    let end_y = self.points.len() - 1;
    let end_x = self.points[0].len() - 2;

    (end_x as i32, end_y as i32)
  }
}

impl fmt::Debug for Map {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut lines = String::from("");
    for line in self.points.iter() {
      for p in line.iter() {
        lines += &p.borrow().to_str();
      }

      lines += "\n";
    }
    write!(f, "{}", lines)
  }
}

pub struct Point {
  pub x: i32,
  pub y: i32,
  pub status: PointStatus,
  pub winds: Vec<WindDir>,
  pub wind_num: usize,
}

impl Point {
  pub fn new(x: i32, y: i32, s: &str) -> Point {
    let status = PointStatus::from_str(s);

    let mut winds = vec![];
    match WindDir::from_str(s) {
      Some(w) => {
        winds.push(w);
      }
      _ => {}
    }

    let wind_num = winds.len();

    Point {
      x,
      y,
      status,
      wind_num,
      winds,
    }
  }
  pub fn is_wall(&self) -> bool {
    match self.status {
      PointStatus::Wall => true,
      _ => false,
    }
  }
  pub fn can_move_to(&self) -> bool {
    match self.status {
      PointStatus::Wall => return false,
      _ => {}
    }
    if self.wind_num >= 1 {
      return false;
    }
    return true;
  }

  pub fn get_raw(&self) -> (i32, i32) {
    (self.x, self.y)
  }

  pub fn add_wind(&mut self, wind: WindDir) {
    self.winds.push(wind);
  }
  pub fn update_wind_num(&mut self) {
    self.wind_num = self.winds.len();
  }
  pub fn to_str(&self) -> String {
    if self.winds.len() > 1 {
      return format!("{}", self.winds.len());
    }
    if self.winds.len() == 1 {
      match self.winds.get(0) {
        Some(w) => return w.to_str(),
        None => panic!("invalid winds"),
      }
    }
    match &self.status {
      PointStatus::Wall => String::from("#"),
      PointStatus::Empty => String::from("."),
    }
  }
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({},{}){}", self.x, self.y, self.to_str())
  }
}

pub enum PointStatus {
  Wall,
  Empty,
}

impl PointStatus {
  pub fn from_str(s: &str) -> PointStatus {
    match s {
      "#" => PointStatus::Wall,
      "." => PointStatus::Empty,
      _ => PointStatus::Empty,
    }
  }
}

#[derive(PartialEq)]
pub enum WindDir {
  North,
  South,
  West,
  East,
}

impl WindDir {
  pub fn from_str(s: &str) -> Option<WindDir> {
    match s {
      "^" => Some(WindDir::North),
      "v" => Some(WindDir::South),
      ">" => Some(WindDir::East),
      "<" => Some(WindDir::West),
      _ => None,
    }
  }
  pub fn to_str(&self) -> String {
    match &self {
      WindDir::North => String::from("^"),
      WindDir::South => String::from("v"),
      WindDir::East => String::from(">"),
      WindDir::West => String::from("<"),
    }
  }
  pub fn get_change(&self) -> (i32, i32) {
    match &self {
      WindDir::North => (0, -1),
      WindDir::South => (0, 1),
      WindDir::East => (1, 0),
      WindDir::West => (-1, 0),
    }
  }
}
