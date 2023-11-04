#[derive(Debug)]
pub enum Dir {
  Right,
  Left,
  Up,
  Down,
}

pub type Move = (Dir, i32);
pub type Distance = (i32, i32);

impl Dir {
  pub fn from_str(dir: &str) -> Self {
    match dir {
      "R" => Dir::Right,
      "L" => Dir::Left,
      "U" => Dir::Up,
      "D" => Dir::Down,
      _ => panic!("not valid dir {}", dir),
    }
  }
  pub fn get_move_unit(dir: &Dir) -> Distance {
    match dir {
      Dir::Right => (1, 0),
      Dir::Left => (-1, 0),
      Dir::Up => (0, 1),
      Dir::Down => (0, -1),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Point(pub i32, pub i32);

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
    Self(x, y)
  }
  pub fn move_to(&self, mo: &Move) -> Self {
    let unit = Dir::get_move_unit(&mo.0);
    let dis = mul_move_unit(unit, mo.1);
    Self(self.0 + dis.0, self.1 + dis.1)
  }
  pub fn get_move_points(&self, mo: &Move) -> Vec<Self> {
    let mut points = Vec::new();
    for i in 1..=mo.1 {
      let unit = Dir::get_move_unit(&mo.0);
      let dis = mul_move_unit(unit, i);
      points.push(Point::new(self.0 + dis.0, self.1 + dis.1));
    }

    points
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1
  }
}

pub fn mul_move_unit(unit: Distance, space: i32) -> Distance {
  (unit.0 * space, unit.1 * space)
}
pub fn get_tail_next(tail_pos: &Point, head_pos: &Point) -> Option<Point> {
  let dis = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);
  let is_near = dis.0.abs() <= 1 && dis.1.abs() <= 1;

  if (is_near) {
    return None;
  }

  let unit_x = if dis.0 == 0 { 0 } else { dis.0 / dis.0.abs() };
  let unit_y = if dis.1 == 0 { 0 } else { dis.1 / dis.1.abs() };

  return Some(Point::new(tail_pos.0 + unit_x, tail_pos.1 + unit_y));
}
