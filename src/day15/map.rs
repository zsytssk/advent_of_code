use std::{
  cell::{RefCell, RefMut},
  fmt,
};

pub struct Map {
  pub sensor_and_bean: Vec<((i32, i32), (i32, i32))>,
}

impl Map {
  pub fn new(info: Vec<(Point, Point)>) -> Self {
    let sensor_and_bean = info
      .iter()
      .map(|item| ((item.0.x, item.0.y), (item.1.x, item.1.y)))
      .collect::<Vec<_>>();

    Map { sensor_and_bean }
  }
}

pub enum PointStatus {
  Empty,
  ConfirmEmpty,
  Sensor,
  Beacon,
}

pub struct Point {
  pub x: i32,
  pub y: i32,
  status: PointStatus,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
    Point {
      x,
      y,
      status: PointStatus::Empty,
    }
  }
  pub fn from_str(s: &str) -> Self {
    let regex = regex::Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
    let m = regex.captures(s).unwrap();
    let (_, match_list): (&str, [&str; 2]) = m.extract();

    Point {
      x: match_list[0].parse().unwrap(),
      y: match_list[1].parse().unwrap(),
      status: PointStatus::Empty,
    }
  }
  pub fn distance_from_point(&self, point: &Point) -> i32 {
    (self.x - point.x).abs() + (self.y - point.y).abs()
  }
  pub fn distance_from_line(&self, line_index: i32) -> i32 {
    (self.y - line_index).abs()
  }
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{},{}", self.x, self.y)
  }
}
