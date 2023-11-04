use std::fmt;

pub struct Map {
  pub points: Vec<Vec<Point>>,
  pub paths: Vec<Path>,
  range_x: (usize, usize),
  range_y: (usize, usize),
}

impl Map {
  pub fn new(paths: Vec<Path>) -> Self {
    let mut map = Map {
      points: Vec::new(),
      paths,
      range_x: (0, 0),
      range_y: (0, 0),
    };
    map.init_points();
    map
  }
  pub fn get_point(&self, pos: &(usize, usize)) -> Option<&Point> {
    if (pos.0 < self.range_x.0 || pos.0 > self.range_x.1)
      || (pos.1 < self.range_y.0 || pos.1 > self.range_y.1)
    {
      return None;
    }
    let x = pos.0 - self.range_x.0;
    let y = pos.1 - self.range_y.0;
    return Some(&self.points[y][x]);
  }
  pub fn get_mut_point(&mut self, pos: &(usize, usize)) -> Option<&mut Point> {
    if (pos.0 < self.range_x.0 || pos.0 > self.range_x.1)
      || (pos.1 < self.range_y.0 || pos.1 > self.range_y.1)
    {
      return None;
    }
    let x = pos.0 - self.range_x.0;
    let y = pos.1 - self.range_y.0;
    return Some(&mut self.points[y][x]);
  }
  pub fn get_sand_points(&self) -> Vec<&Point> {
    let mut list = Vec::new();
    for line in self.points.iter() {
      for point in line.iter() {
        match point.status {
          PointStatus::Sand => {
            list.push(point);
          }
          _ => continue,
        }
      }
    }

    list
  }
  pub fn init_points(&mut self) {
    let mut min_x = 500;
    let mut max_x = 500;
    let mut max_y = 0;
    for path in self.paths.iter() {
      for point in path.points.iter() {
        if (min_x > point.x) {
          min_x = point.x;
        }
        if (max_x < point.x) {
          max_x = point.x;
        }
        if (max_y < point.y) {
          max_y = point.y;
        }
      }
    }
    for y in 0..=max_y {
      let mut points = Vec::new();
      for x in min_x..=max_x {
        let mut p = Point::new(x, y);
        let p_in_path = self.point_on_path(&p);
        if p_in_path {
          p.set_status(PointStatus::Wall)
        }
        points.push(p);
      }
      self.points.push(points);
    }

    self.range_x = (min_x, max_x);
    self.range_y = (0, max_y);
  }
  pub fn point_on_path(&self, point: &Point) -> bool {
    for path in self.paths.iter() {
      for i in 0..path.points.len() - 1 {
        if path.points[i].x == path.points[i + 1].x {
          if point.x == path.points[i].x {
            let mut arr = vec![path.points[i].y, path.points[i + 1].y];
            arr.sort();
            if point.y >= arr[0] && point.y <= arr[1] {
              return true;
            }
          }
        } else if path.points[i].y == path.points[i + 1].y {
          if point.y == path.points[i].y {
            let mut arr = vec![path.points[i].x, path.points[i + 1].x];
            arr.sort();
            if point.x >= arr[0] && point.x <= arr[1] {
              return true;
            }
          }
        }
      }
    }
    false
  }
  pub fn add_bottom(&mut self, space: usize) {
    let range_x = self.range_x;
    let range_y = self.range_y;

    let bottom_y = range_y.1 + 2;
    let new_range_x = (range_x.0 - space, range_x.1 + space);
    let new_range_y = (0, range_y.1 + 2);

    for y in new_range_y.0..=new_range_y.1 {
      let is_old = y <= range_y.1;
      let mut line_holder: Vec<Point> = Vec::new();
      let mut line = &mut line_holder;
      if is_old {
        line = &mut self.points[y as usize];
      }
      for x in new_range_x.0..=new_range_x.1 {
        if is_old == false {
          let mut p = Point::new(x, y);
          if y == new_range_y.1 {
            p.set_status(PointStatus::Wall);
          }
          line.push(p);
        } else {
          if x < range_x.0 {
            line.insert(0, Point::new(x, y));
          } else if x > range_x.1 {
            line.push(Point::new(x, y));
          }
        }
      }

      if is_old == false {
        self.points.push(line_holder);
      }
    }

    self.range_x = new_range_x;
    self.range_y = new_range_y;
  }
}

impl fmt::Debug for Map {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut list = String::from("");
    for line in self.points.iter() {
      for point in line.iter() {
        list.push_str(&point.to_map_string());
      }
      list.push_str("\n");
    }

    write!(f, "{}", list)
  }
}

pub struct Path {
  pub points: Vec<Point>,
}

impl Path {
  pub fn new(points: Vec<Point>) -> Self {
    Path { points }
  }
}

impl fmt::Debug for Path {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut list = Vec::new();
    for item in self.points.iter() {
      list.push(format!("{:?}", item.to_string()));
    }

    write!(f, "{}", list.join(" -> "))
  }
}

#[derive(Debug)]
pub struct Point {
  pub y: usize,
  pub x: usize,
  pub status: PointStatus,
}

#[derive(Debug)]
pub enum PointStatus {
  Empty,
  Wall,
  Sand,
}

impl Point {
  pub fn new(x: usize, y: usize) -> Self {
    Point {
      x,
      y,
      status: PointStatus::Empty,
    }
  }
  pub fn from_str(s: &str) -> Self {
    let arr = s
      .split(",")
      .map(|x| x.parse::<usize>().unwrap())
      .collect::<Vec<_>>();

    if arr.len() < 2 {
      panic!("parse Point error");
    }

    Point {
      x: arr[0],
      y: arr[1],
      status: PointStatus::Empty,
    }
  }
  pub fn set_status(&mut self, status: PointStatus) {
    self.status = status;
  }
  pub fn can_move_to(&self) -> bool {
    match self.status {
      PointStatus::Empty => true,
      _ => false,
    }
  }
  pub fn to_string(&self) -> String {
    format!("{},{}", self.x, self.y)
  }
  pub fn to_map_string(&self) -> String {
    let s = match self.status {
      PointStatus::Empty => ".",
      PointStatus::Sand => "o",
      PointStatus::Wall => "#",
    };
    String::from(s)
  }
}
