use std::{collections::HashMap, time::Instant};

use super::operate::*;

pub fn parse_step1(
  statements: &Vec<Statement>,
) -> (HashMap<String, i64>, Vec<String>) {
  let mut save_arr: Vec<String> = vec![];
  let mut res_map: HashMap<String, i64> = HashMap::new();

  for item in statements.iter() {
    match &item.op {
      OperateWrap::Number(num) => {
        res_map.insert(item.name.clone(), num.clone());
      }
      OperateWrap::Unknown => {
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
    let loop_arr = calc_top_step1(&mut save_arr, &res_map, &statements);

    if loop_arr.len() == 0 {
      break;
    }

    for item in loop_arr.iter() {
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
  }

  return (res_map, save_arr);
}

pub fn calc_top_step1(
  save_arr: &mut Vec<String>,
  res_map: &HashMap<String, i64>,
  statements: &Vec<Statement>,
) -> Vec<String> {
  let mut arr = vec![];
  save_arr.retain(|item| {
    let statement = statements.iter().find(|x| x.name == *item).unwrap();
    match &statement.op {
      OperateWrap::Operate(op) => {
        let left = res_map.get(&op.left);
        let right = res_map.get(&op.right);
        if left.is_none() || right.is_none() {
          return true;
        }

        arr.push(item.clone());
        return false;
      }
      _ => panic!(),
    };
  });

  arr
}

pub fn parse_step2(
  save_arr: &mut Vec<String>,
  res_map: &mut HashMap<String, i64>,
  statements: &Vec<Statement>,
) {
  loop {
    let loop_arr = calc_top_step2(save_arr, res_map, statements);

    if loop_arr.len() == 0 && save_arr.len() == 0 {
      break;
    }
  }
}

pub fn calc_top_step2(
  save_arr: &mut Vec<String>,
  res_map: &HashMap<String, i64>,
  statements: &Vec<Statement>,
) -> Vec<String> {
  let mut arr = vec![];
  save_arr.retain(|item| {
    let statement = statements.iter().find(|x| x.name == *item).unwrap();
    match &statement.op {
      OperateWrap::Operate(op) => {
        let left = res_map.get(&op.left);
        let right = res_map.get(&op.right);
        if left.is_none() || right.is_none() {
          return true;
        }

        arr.push(item.clone());
        return false;
      }
      _ => panic!(),
    };
  });

  arr
}
