#![allow(unused)]
use crate::utils::read_file;

use self::snafu::Snafu;

mod snafu;
mod utils;

pub fn parse() {
  parse1();
  // parse2();
}

fn parse1() {
  let list = parse_input();

  let sum: i64 = list.iter().map(|item| item.to_decimal()).sum();

  println!("{:?}", sum);
}

fn parse_input() -> Vec<Snafu> {
  let content = read_file("day25/demo.txt").unwrap();

  content
    .split("\n")
    .map(|item| Snafu::new(item))
    .collect::<Vec<Snafu>>()
}
