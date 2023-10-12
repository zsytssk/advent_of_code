use std::{
    cell::{RefCell, RefMut},
    fmt,
};

pub struct Map {
    pub points: Vec<Vec<RefCell<Point>>>,
    pub range_x: (i32, i32),
    pub range_y: (i32, i32),
    pub sensor_and_bean: Vec<((i32, i32), (i32, i32))>,
}

impl Map {
    pub fn new(range: ((i32, i32), (i32, i32))) -> Self {
        let mut points = Vec::new();
        for y in range.1 .0..=range.1 .1 {
            let mut line = Vec::new();
            for x in range.0 .0..=range.0 .1 {
                line.push(RefCell::new(Point::new(x, y)));
            }
            points.push(line);
        }

        Map {
            points,
            range_x: range.0,
            range_y: range.1,
            sensor_and_bean: Vec::new(),
        }
    }
    pub fn get_line_y(line_index: i32) -> i32 {
        todo!()
    }
    pub fn set_sensor_and_bean(&mut self, info: Vec<(Point, Point)>) {
        for item in info.iter() {
            self.set_point((item.0.x, item.0.y), PointStatus::Sensor);
            self.set_point((item.1.x, item.1.y), PointStatus::Beacon);
        }

        self.sensor_and_bean = info
            .iter()
            .map(|item| ((item.0.x, item.0.y), (item.1.x, item.1.y)))
            .collect::<Vec<_>>();
    }
    pub fn set_point(&self, pos: (i32, i32), status: PointStatus) {
        if (pos.0 < self.range_x.0 || pos.0 > self.range_x.1)
            || (pos.1 < self.range_y.0 || pos.1 > self.range_y.1)
        {
            return;
        }

        let x = (pos.0 - self.range_x.0) as usize;
        let y = (pos.1 - self.range_y.0) as usize;

        self.points[y][x].borrow_mut().set_status(status);
    }
    pub fn get_point(&self, pos: (i32, i32)) -> Option<RefMut<Point>> {
        if (pos.0 < self.range_x.0 || pos.0 > self.range_x.1)
            || (pos.1 < self.range_y.0 || pos.1 > self.range_y.1)
        {
            return None;
        }

        let x = (pos.0 - self.range_x.0) as usize;
        let y = (pos.1 - self.range_y.0) as usize;

        Some(self.points[y][x].borrow_mut())
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = String::from("");
        for line in self.points.iter() {
            for point in line.iter() {
                list.push_str(&point.borrow().to_map_string());
            }
            list.push_str("\n");
        }

        write!(f, "{}", list)
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
    pub fn set_status(&mut self, status: PointStatus) {
        self.status = status;
    }
    pub fn to_map_string(&self) -> String {
        let s = match self.status {
            PointStatus::Empty => ".",
            PointStatus::ConfirmEmpty => "#",
            PointStatus::Sensor => "S",
            PointStatus::Beacon => "B",
        };
        String::from(s)
    }
    pub fn is_not_empty(&self) -> bool {
        match self.status {
            PointStatus::Empty => false,
            PointStatus::ConfirmEmpty => false,
            PointStatus::Sensor => true,
            PointStatus::Beacon => true,
        }
    }
    pub fn distance_from_point(&self, point: RefMut<Point>) -> i32 {
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
