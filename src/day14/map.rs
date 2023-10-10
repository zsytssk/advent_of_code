use std::fmt;

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
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point(x, y)
    }
    pub fn from_str(s: &str) -> Self {
        let arr = s
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if arr.len() < 2 {
            panic!("parse Point error");
        }

        Point(arr[0], arr[1])
    }
    pub fn to_string(&self) -> String {
        format!("{},{}", self.0, self.1)
    }
}
