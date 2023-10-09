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

#[derive(Debug)]
pub enum ListItem {
    Num(i32),
    List(Vec<ListItem>),
}

impl ListItem {
    pub fn parse() -> Self {
        ListItem::Num(0)
    }
    pub fn get_num(num: i32) -> Self {
        ListItem::Num(num)
    }
}
