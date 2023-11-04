#![allow(unused)]
use crate::utils::read_file;

pub fn parse() {
  parse1();
  parse2();
}

fn parse1() {
  let content = read_file("day3/input.txt").unwrap();
  let sum: u32 = content
    .split("\n")
    .map(|line| {
      let len = line.len();
      let first = &line[0..len / 2];
      let second = &line[len / 2..len];
      for c in first.chars() {
        if second.contains(c) {
          return c;
        }
      }
      return ' ';
    })
    .map(|f| get_char_score(f))
    .sum();

  println!("{:?}", sum);
}

fn parse2() {
  let content = read_file("day3/input.txt").unwrap();
  let lines: Vec<&str> = content.split("\n").collect();
  let sum: u32 = lines
    .chunks(3)
    .map(|item| {
      let [first, second, third] = item else {
        panic!("Invalid item")
      };
      for c in first.chars() {
        if second.contains(c) && third.contains(c) {
          return c;
        }
      }
      return ' ';
    })
    .map(|c| get_char_score(c))
    .sum();

  println!("{:?}", sum);
}

fn get_char_score(c: char) -> u32 {
  match c {
    'a'..='z' => c as u32 - 'a' as u32 + 1,
    'A'..='Z' => c as u32 - 'A' as u32 + 27,
    _ => 0,
  }
}
