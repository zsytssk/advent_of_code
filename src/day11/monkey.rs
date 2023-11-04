type Num = u128;

#[derive(Debug)]
pub struct Monkey {
  pub inspected_items: Num,
  pub index: Num,
  pub items: Vec<Num>,
  pub opt: Operation,
  pub div: Num,
  pub if_true: Num,
  pub if_false: Num,
}

impl Monkey {
  pub fn from_str(str: &str) -> Self {
    let monkey_reg = regex::Regex::new(
      r"Monkey (\d+):
  Starting items: ([^\n]+)
  Operation: new = old ([\*|\+]) (\d+|old)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
    )
    .unwrap();

    let m = monkey_reg.captures(str).unwrap();
    let (_, match_list): (&str, [&str; 7]) = m.extract();

    let index: Num = match_list[0].parse().unwrap();
    let items: Vec<Num> = match_list[1]
      .split(", ")
      .map(|x| x.parse().unwrap())
      .collect();
    let div: Num = match_list[4].parse().unwrap();
    let if_true: Num = match_list[5].parse().unwrap();
    let if_false: Num = match_list[6].parse().unwrap();

    let opt = match (match_list[2], match_list[3]) {
      ("*", "old") => Operation::MulSelf,
      ("+", "old") => Operation::AddSelf,
      ("+", s) => Operation::Add(s.parse::<Num>().unwrap()),
      ("*", s) => Operation::Mul(s.parse::<Num>().unwrap()),
      _ => panic!("unknown operation"),
    };

    Monkey {
      inspected_items: 0,
      index,
      items,
      opt,
      div,
      if_true,
      if_false,
    }
  }
  pub fn run(&mut self, div_three: bool, modulus: Num) -> Vec<(Num, Num)> {
    let mut res: Vec<(Num, Num)> = Vec::new();

    while self.items.len() > 0 {
      let item = self.items.remove(0);
      let mut num = self.opt.apply(&item);
      if (div_three) {
        num = num / 3;
      }
      num %= modulus;

      if is_divisible(num, self.div) {
        res.push((self.if_true, num));
      } else {
        res.push((self.if_false, num));
      }
      self.inspected_items += 1;
    }

    res
  }
  pub fn add_num_list(&mut self, num: Num) {
    self.items.push(num);
  }
  pub fn get_inspected_items(&self) -> Num {
    self.inspected_items.clone()
  }
}

#[derive(Debug)]
pub enum Operation {
  MulSelf,
  AddSelf,
  Mul(Num),
  Add(Num),
}

impl Operation {
  pub fn apply(&self, old: &Num) -> Num {
    match self {
      Operation::MulSelf => old * old,
      Operation::AddSelf => old + old,
      Operation::Add(n) => old + n,
      Operation::Mul(n) => old * n,
    }
  }
}

pub fn is_divisible(num: Num, div: Num) -> bool {
  let time = num / div;
  time * div == num
}
