#![allow(unused)]
use std::{collections::HashMap, time::Instant};

use crate::utils::read_file;

mod operate;
mod utils;
use operate::*;

pub fn parse() {
  parse1();
  // parse2();
}

fn parse1() {
  let statements = parse_input();
  let now = Instant::now();

  let (res_map, _) = utils::parse_step1(&statements);
  println!(
    "now={:?} | res={:?}",
    now.elapsed(),
    res_map.get("root").unwrap()
  );
}

fn parse2() {
  let mut statements = parse_input();
  let now = Instant::now();

  let root = statements
    .iter_mut()
    .find(|item| item.name == "root")
    .unwrap();

  match &mut root.op {
    OperateWrap::Operate(n) => {
      n.opr = Operator::Equal;
    }
    _ => unreachable!(),
  };

  let human = statements
    .iter_mut()
    .find(|item| item.name == "humn")
    .unwrap();

  human.op = OperateWrap::Unknown;

  let mut save_arr: Vec<String> = vec![];

  let (mut res_map, mut save_arr) = utils::parse_step1(&statements);
  let root = statements
    .iter_mut()
    .find(|item| item.name == "root")
    .unwrap();

  match &mut root.op {
    OperateWrap::Operate(n) => {
      let left = res_map.get(&n.left);
      let right = res_map.get(&n.right);

      if right.is_some() {
        let val = right.unwrap().clone();
        res_map.insert(n.left.clone(), val.clone());
      } else if left.is_some() {
        let val = left.unwrap().clone();
        res_map.insert(n.right.clone(), val.clone());
      }
    }
    _ => unreachable!(),
  };

  utils::parse_step2(&mut save_arr, &mut res_map, &statements);

  let humn = res_map.get("humn").unwrap();

  println!("time={:?} | res_map={:?}", now.elapsed(), humn);
}

fn parse_input() -> Vec<Statement> {
  let content = read_file("day21/input.txt").unwrap();

  content.split("\n").map(Statement::form_str).collect()
}
