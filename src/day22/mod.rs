#![allow(unused)]
use crate::utils::read_file;

pub fn parse() {
  parse1();
  // parse2();
}

fn parse1() {
  let content = parse_input();
}

fn parse_input() -> String {
  let content = read_file("day22/input.txt").unwrap();

  content
}
