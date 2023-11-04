#[derive(Debug)]
pub enum Operator {
  Add,
  Minus,
  Multiply,
  Divide,
  Equal,
}

impl Operator {
  pub fn form_str(s: &str) -> Self {
    match s {
      "+" => Operator::Add,
      "-" => Operator::Minus,
      "*" => Operator::Multiply,
      "/" => Operator::Divide,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub struct Operate {
  pub left: String,
  pub opr: Operator,
  pub right: String,
}

impl Operate {
  pub fn form_str(s: &str) -> Self {
    let mut split = s.split(" ").collect::<Vec<_>>();
    let left = split[0].to_string();
    let opr = Operator::form_str(split[1]);
    let right = split[2].to_string();

    Operate { left, opr, right }
  }
}

#[derive(Debug)]
pub enum OperateWrap {
  Number(i64),
  Unknown,
  Operate(Operate),
}

impl OperateWrap {
  pub fn form_str(s: &str) -> Self {
    if let Ok(num) = s.parse::<i64>() {
      OperateWrap::Number(num)
    } else {
      OperateWrap::Operate(Operate::form_str(s))
    }
  }
}

#[derive(Debug)]
pub struct Statement {
  pub name: String,
  pub op: OperateWrap,
}

impl Statement {
  pub fn form_str(s: &str) -> Self {
    let mut split = s.split(": ").collect::<Vec<_>>();
    let name = split[0].to_string();
    let op = OperateWrap::form_str(split[1]);
    Statement { name, op }
  }
}
