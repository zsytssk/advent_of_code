#[derive(Debug)]
pub struct Monkey {
    inspected_items: u32,
    index: u32,
    items: Vec<u32>,
    opt: Operation,
    div: u32,
    if_true: u32,
    if_false: u32,
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

        let index: u32 = match_list[0].parse().unwrap();
        let items: Vec<u32> = match_list[1]
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let div: u32 = match_list[4].parse().unwrap();
        let if_true: u32 = match_list[5].parse().unwrap();
        let if_false: u32 = match_list[6].parse().unwrap();

        let opt = match (match_list[2], match_list[3]) {
            ("*", "old") => Operation::MulSelf,
            ("+", "old") => Operation::AddSelf,
            ("+", s) => Operation::Add(s.parse::<u32>().unwrap()),
            ("*", s) => Operation::Mul(s.parse::<u32>().unwrap()),
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
    pub fn run(&mut self) -> Vec<(u32, u32)> {
        let mut res: Vec<(u32, u32)> = Vec::new();

        while self.items.len() > 0 {
            let item = self.items.remove(0);
            let mut num = self.opt.apply(&item);
            // num = num / 3;
            if is_divisible(num, self.div) {
                res.push((self.if_true, num));
            } else {
                res.push((self.if_false, num));
            }
            self.inspected_items += 1;
        }

        res
    }
    pub fn add_num_list(&mut self, num: u32) {
        self.items.push(num);
    }
    pub fn get_inspected_items(&self) -> u32 {
        self.inspected_items.clone()
    }
}

#[derive(Debug)]
pub enum Operation {
    MulSelf,
    AddSelf,
    Mul(u32),
    Add(u32),
}

impl Operation {
    pub fn apply(&self, old: &u32) -> u32 {
        match self {
            Operation::MulSelf => old * old,
            Operation::AddSelf => old + old,
            Operation::Add(n) => old + n,
            Operation::Mul(n) => old * n,
        }
    }
}

fn is_divisible(num: u32, div: u32) -> bool {
    let time = num / div;
    time * div == num
}
