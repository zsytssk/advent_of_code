#![allow(unused)]
use crate::utils::read_file;

// https://adventofcode.com/2022/day/4
pub fn parse() {
  parse1();
  parse2();
}

fn parse1() {
  let content = read_file("day4/input.txt").unwrap();
  let sum: i32 = content
    .split("\n")
    .map(|item| {
      let [zone1, zone2] =
        if let [zone1, zone2] = item.split(",").collect::<Vec<&str>>()[..] {
          [zone1, zone2]
        } else {
          panic!("can't parse line");
        };

      let [first1, end1] = if let [first1, end1] = zone1
        .split("-")
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
      {
        [first1, end1]
      } else {
        panic!("can't parse zone1");
      };

      let [first2, end2] = if let [first2, end2] = zone2
        .split("-")
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
      {
        [first2, end2]
      } else {
        panic!("can't parse zone2");
      };

      if (first1 >= first2 && end1 <= end2)
        || (first1 <= first2 && end1 >= end2)
      {
        return 1;
      }
      return 0;
    })
    .sum();

  println!("parse1 sum: {}", sum);
}

fn parse2() {
  let content = read_file("day4/input.txt").unwrap();
  let sum: i32 = content
    .split("\n")
    .map(|item| {
      let [zone1, zone2] =
        if let [zone1, zone2] = item.split(",").collect::<Vec<&str>>()[..] {
          [zone1, zone2]
        } else {
          panic!("can't parse line");
        };

      let [first1, end1] = if let [first1, end1] = zone1
        .split("-")
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
      {
        [first1, end1]
      } else {
        panic!("can't parse zone1");
      };

      let [first2, end2] = if let [first2, end2] = zone2
        .split("-")
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..]
      {
        [first2, end2]
      } else {
        panic!("can't parse zone2");
      };

      if first1 > end2 || end1 < first2 {
        return 0;
      }
      return 1;
    })
    .sum();

  println!("parse2 sum: {}", sum);
}
