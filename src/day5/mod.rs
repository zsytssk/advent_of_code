#![allow(unused)]
use std::vec::Drain;

use crate::utils::read_file;
use regex::Regex;

// 如果有一个简单的match [\w]\s+[\w] 就好了
// 取最后一行的字母，然后用他的index取其他行的内容
// https://adventofcode.com/2022/day/5
pub fn parse() {
  // parse1();
  parse2();
}

fn parse1() {
  let (mut stack, procedure) = parse_input();

  for item in procedure {
    let [num, from, to] = item;
    for i in 0..num {
      let c = stack[from - 1].remove(0);
      stack[to - 1].insert(0, c);
    }
  }
  let str = stack.iter().map(|item| item[0]).collect::<String>();
  println!("stack {:?}", str);
}

fn parse2() {
  let (mut stack, procedure) = parse_input();

  for item in procedure {
    let [num, from, to] = item;
    let remove_arr = stack[from - 1].drain(0..num).collect::<Vec<_>>();
    stack[to - 1].splice(0..0, remove_arr);
  }
  let str = stack.iter().map(|item| item[0]).collect::<String>();
  println!("stack {:?}", str);
}

fn parse_input() -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
  let content = read_file("day5/input.txt").unwrap();
  let arr = content.split("\n\n").collect::<Vec<&str>>();
  let stack_str = arr[0];
  let procedure_str = arr[1];

  let stack = get_stack(stack_str);

  let procedure: Vec<[usize; 3]> =
    procedure_str.split("\n").map(|x| get_num(x)).collect();

  (stack, procedure)
}

fn get_stack(str: &str) -> Vec<Vec<char>> {
  let stack_lines = str.split("\n").collect::<Vec<&str>>();
  let last_line = stack_lines[stack_lines.len() - 1];
  let line_num = stack_lines.len();

  let mut stack: Vec<Vec<char>> = Vec::new();
  for i in 0..last_line.len() {
    let mut column = Vec::new();
    let char = last_line.chars().nth(i).unwrap();
    if char != ' ' {
      for j in 0..line_num - 1 {
        let char_res = stack_lines[j].chars().nth(i);

        match stack_lines[j].chars().nth(i) {
          Some(c) => {
            if c != ' ' {
              column.push(c);
            }
          }
          None => continue,
        }
      }
    }
    if column.len() != 0 {
      stack.push(column);
    }
  }

  return stack;
}
fn get_num(str: &str) -> [usize; 3] {
  let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  let m = re.captures(str).unwrap();
  let (_, num_list): (&str, [&str; 3]) = m.extract();

  return num_list.map(|num_str| num_str.parse::<usize>().unwrap());
}
