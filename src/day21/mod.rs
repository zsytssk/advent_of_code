#![allow(unused)]
use std::{collections::HashMap, time::Instant};

use crate::utils::read_file;

mod operate;
use operate::*;

pub fn parse() {
  parse1();
  // parse2();
}

fn parse1() {
  let statements = parse_input();
  let now = Instant::now();

  let mut loop_arr: Vec<String> = vec![];
  let mut save_arr: Vec<String> = vec![];
  let mut res_map: HashMap<String, i64> = HashMap::new();

  for item in statements.iter() {
    match &item.op {
      OperateWrap::Number(num) => {
        res_map.insert(item.name.clone(), num.clone());
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

    let mut remove_list = vec![];
    let mut add_list = vec![];
    for (index, item) in loop_arr.iter().enumerate() {
      match res_map.get(item) {
        Some(n) => {
          remove_list.push(index);
          continue;
        }
        None => {}
      };

      let s = statements.iter().find(|x| x.name == *item).unwrap();
      let num = match &s.op {
        OperateWrap::Number(n) => n.clone(),
        OperateWrap::Operate(op) => {
          let left = res_map.get(&op.left);
          let right = res_map.get(&op.right);
          if left.is_none() || right.is_none() {
            if left.is_none() {
              add_list.push(op.left.clone());
            }
            if right.is_none() {
              add_list.push(op.right.clone());
            }
            continue;
          }

          match &op.opr {
            Operator::Add => left.unwrap() + right.unwrap(),
            Operator::Minus => left.unwrap() - right.unwrap(),
            Operator::Multiply => left.unwrap() * right.unwrap(),
            Operator::Divide => left.unwrap() / right.unwrap(),
          }
        }
      };

      res_map.insert(item.clone(), num);
      remove_list.push(index);
    }

    loop_arr.clear();

    if loop_arr.len() == 0 && save_arr.len() == 0 {
      break;
    }
  }

  println!(
    "now={:?} | res={:?}",
    now.elapsed(),
    res_map.get("root").unwrap()
  );
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
