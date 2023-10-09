use std::{cmp::Ordering, fmt};

#[derive(Debug)]
pub enum Token {
    Num(i32),
    Dot,
    EndOfArr,
    StartOfArr,
}

impl Token {
    pub fn new(s: char) -> Self {
        match s {
            '[' => Token::StartOfArr,
            ']' => Token::EndOfArr,
            ',' => Token::Dot,
            x => Token::Num(x.to_digit(10).unwrap() as i32),
        }
    }
    pub fn append_num(&mut self, num: &str) {
        match self {
            Token::Num(x) => {
                *x = format!("{}{}", x, num).parse::<i32>().unwrap()
            }
            Token::Dot => {}
            Token::EndOfArr => {}
            Token::StartOfArr => {}
        }
    }
}

// #[derive(Debug)]
pub enum ListItem {
    Num(i32),
    List(Vec<ListItem>),
}

impl fmt::Debug for ListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement how you want to format your struct for debugging
        match &self {
            ListItem::Num(x) => write!(f, "{}", x),
            ListItem::List(arr) => {
                let mut list = Vec::new();
                for item in arr.iter() {
                    list.push(format!("{:?}", item));
                }

                // write!(f, "{}{}{}", "[", list.join(","), "]")
                write!(f, "{}{}{}", "[", list.join(","), "]")
            }
        }
    }
}

impl ListItem {
    pub fn parse() -> Self {
        ListItem::Num(0)
    }
    pub fn get_num(num: i32) -> Self {
        ListItem::Num(num)
    }
    pub fn comp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListItem::Num(x), ListItem::Num(y)) => x.cmp(y),
            (ListItem::Num(x), ListItem::List(y)) => {
                ListItem::List(vec![ListItem::Num(*x)]).comp(other)
            }
            (ListItem::List(x), ListItem::Num(y)) => {
                self.comp(&ListItem::List(vec![ListItem::Num(*y)]))
            }
            (ListItem::List(x), ListItem::List(y)) => {
                for (a, b) in x.iter().zip(y.iter()) {
                    let r = a.comp(b);
                    if r != Ordering::Equal {
                        return r;
                    }
                }
                x.len().cmp(&y.len())
            }
        }
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.comp(other)
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.comp(other) == Ordering::Equal
    }
}
impl Eq for ListItem {}
