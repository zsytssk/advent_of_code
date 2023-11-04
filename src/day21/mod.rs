#![allow(unused)]
use std::{collections::HashMap, time::Instant};

use crate::utils::read_file;

mod operate;
use operate::*;

pub fn parse() {
  // parse1();
  parse2();
}

fn parse1() {
  let statements = parse_input();
  let now = Instant::now();

  let (res_map, _) = parse_step1(&statements);
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

  let (mut res_map, save_arr) = parse_step1(&statements);
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
}

fn parse_step1(
  statements: &Vec<Statement>,
) -> (HashMap<String, i64>, Vec<String>) {
  let mut loop_arr: Vec<String> = vec![];
  let mut save_arr: Vec<String> = vec![];
  let mut res_map: HashMap<String, i64> = HashMap::new();

  for item in statements.iter() {
    match &item.op {
      OperateWrap::Number(num) => {
        res_map.insert(item.name.clone(), num.clone());
      }
      OperateWrap::Unknown => {
        save_arr.push(item.name.clone());
        continue;
      }
      OperateWrap::Operate(op) => {
        save_arr.push(item.name.clone());
        continue;
      }
    }
  }

  println!(
    "res_map_len={:?} | statements={:?}",
    res_map.len(),
    statements.len()
  );

  loop {
    calc_top_list(&mut loop_arr, &mut save_arr, &res_map, &statements);
    if loop_arr.len() == 0 {
      break;
    }

    for (index, item) in loop_arr.iter().enumerate() {
      match res_map.get(item) {
        Some(n) => {
          continue;
        }
        None => {}
      };

      let s = statements.iter().find(|x| x.name == *item).unwrap();
      let num = match &s.op {
        OperateWrap::Number(n) => n.clone(),
        OperateWrap::Unknown => unreachable!(),
        OperateWrap::Operate(op) => {
          let left = res_map.get(&op.left);
          let right = res_map.get(&op.right);
          if left.is_none() || right.is_none() {
            continue;
          }

          match &op.opr {
            Operator::Add => left.unwrap() + right.unwrap(),
            Operator::Minus => left.unwrap() - right.unwrap(),
            Operator::Multiply => left.unwrap() * right.unwrap(),
            Operator::Divide => left.unwrap() / right.unwrap(),
            Operator::Equal => panic!("equal"),
          }
        }
      };

      res_map.insert(item.clone(), num);
    }

    loop_arr.clear();
  }

  return (res_map, save_arr);
}

fn calc_top_list(
  loop_arr: &mut Vec<String>,
  save_arr: &mut Vec<String>,
  res_map: &HashMap<String, i64>,
  statements: &Vec<Statement>,
) {
  save_arr.retain(|item| {
    let statement = statements.iter().find(|x| x.name == *item).unwrap();
    match &statement.op {
      OperateWrap::Operate(op) => {
        let left = res_map.get(&op.left);
        let right = res_map.get(&op.right);
        if left.is_none() || right.is_none() {
          return true;
        }

        loop_arr.push(item.clone());
        return false;
      }
      _ => panic!(),
    };
  });
}

fn parse_input() -> Vec<Statement> {
  let content = read_file("day21/input.txt").unwrap();

  content.split("\n").map(Statement::form_str).collect()
}
