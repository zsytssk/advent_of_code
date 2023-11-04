use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum SurfaceStatus {
  Outer,
  Overlap,
  Inner,
}
pub struct CubeList {
  cubes: Vec<Cube>,
}

impl CubeList {
  pub fn new(cubes: Vec<Cube>) -> Self {
    CubeList { cubes }
  }
}

#[derive(PartialEq, Hash, Eq)]
pub struct Cube {
  pub points: Vec<Point>,
  pub surfaces: Vec<Surface>,
  pub x: usize,
  pub y: usize,
  pub z: usize,
}

impl Cube {
  pub fn new(x: usize, y: usize, z: usize) -> Self {
    let mut points = vec![];
    for x in x..=x + 1 {
      for y in y..=y + 1 {
        for z in z..=z + 1 {
          let point = Point::new(x, y, z);
          points.push(point);
        }
      }
    }
    let mut cube = Cube {
      points,
      x,
      y,
      z,
      surfaces: vec![],
    };

    cube.init_surfaces();

    cube
  }
  pub fn is_overlap(&self, other: &Cube) -> bool {
    let mut overlap_points_num = 0;

    for point in self.points.iter() {
      if other.contain_point(point) {
        overlap_points_num += 1
      }
    }

    overlap_points_num == 4
  }
  pub fn contain_point(&self, point: &Point) -> bool {
    self.points.contains(point)
  }
  pub fn init_surfaces(&mut self) {
    let points = &self.points;
    let x = self.x.clone();
    let y = self.y.clone();
    let z = self.z.clone();

    let mut points1 = vec![];
    let mut points2 = vec![];
    for x in x..=x + 1 {
      for y in y..=y + 1 {
        let point1 = Point::new(x, y, z);
        points1.push(point1);

        let point2 = Point::new(x, y, z + 1);
        points2.push(point2);
      }
    }
    let surface1 = Surface::new(points1);
    let surface2 = Surface::new(points2);

    let mut points3 = vec![];
    let mut points4 = vec![];
    for x in x..=x + 1 {
      for z in z..=z + 1 {
        let point3 = Point::new(x, y, z);
        points3.push(point3);

        let point4 = Point::new(x, y + 1, z);
        points4.push(point4);
      }
    }

    let surface3 = Surface::new(points3);
    let surface4 = Surface::new(points4);

    let mut points5 = vec![];
    let mut points6 = vec![];
    for y in y..=y + 1 {
      for z in z..=z + 1 {
        let point5 = Point::new(x, y, z);
        points5.push(point5);

        let point6 = Point::new(x + 1, y, z);
        points6.push(point6);
      }
    }

    let surface5 = Surface::new(points5);
    let surface6 = Surface::new(points6);

    self.surfaces.push(surface1);
    self.surfaces.push(surface2);
    self.surfaces.push(surface3);
    self.surfaces.push(surface4);
    self.surfaces.push(surface5);
    self.surfaces.push(surface6);
  }
}

impl fmt::Debug for Cube {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{},{},{}", self.x, self.y, self.z)
  }
}

#[derive(PartialEq, Hash, Eq)]
pub struct Surface {
  points: Vec<Point>,
}

impl Surface {
  pub fn new(points: Vec<Point>) -> Self {
    Surface { points }
  }
  pub fn is_overlap(&self, other: &Surface) -> bool {
    let mut overlap_points_num = 0;

    for point in self.points.iter() {
      if other.contain_point(point) {
        overlap_points_num += 1
      }
    }

    overlap_points_num == 4
  }
  pub fn is_cross(&self, other: &Surface) -> bool {
    let mut overlap_points_num = 0;

    for point in self.points.iter() {
      if other.contain_point(point) {
        overlap_points_num += 1
      }
    }

    overlap_points_num == 2
  }
  pub fn contain_point(&self, point: &Point) -> bool {
    self.points.contains(point)
  }
  pub fn fmt_str(&self) -> String {
    let points = &self.points;
    format!(
      "{},{},{}->{},{},{}",
      points[0].x,
      points[0].y,
      points[0].z,
      points[3].x,
      points[3].y,
      points[3].z
    )
  }
}

impl fmt::Debug for Surface {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "surface:{}", self.fmt_str())
  }
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Point {
  pub x: usize,
  pub y: usize,
  pub z: usize,
}

impl Point {
  pub fn new(x: usize, y: usize, z: usize) -> Self {
    Point { x, y, z }
  }
}
