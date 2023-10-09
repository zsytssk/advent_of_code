#![allow(unused)]
use crate::utils::read_file;

mod list;
use list::*;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let content = parse_input();
    println!("{:?}", content);
}

fn parse_input() -> Vec<(ListItem, ListItem)> {
    let content = read_file("day13/demo.txt").unwrap();

    content
        .split("\n\n")
        .map(|block| {
            let mut top_bottom = block.split("\n").collect::<Vec<_>>();
            let top = top_bottom.remove(0);
            let bottom = top_bottom.remove(0);
            (parse_line(top), parse_line(bottom))
        })
        .collect::<Vec<_>>()
}

pub fn parse_line(str: &str) -> ListItem {
    let items = str.split("").filter(|&c| c != "").collect::<Vec<_>>();
    let mut local_str = str.clone();
    let mut tokens = Vec::new();
    loop {
        if local_str.len() == 0 {
            break;
        }
        let (len, token) = peek(&local_str);
        local_str = &local_str[len..local_str.len()];
        tokens.push(token);
    }
    let (_, arr) = parse_tokens(0, &tokens);

    arr
}

pub fn parse_tokens(
    start_index: usize,
    tokens: &Vec<Token>,
) -> (usize, ListItem) {
    let mut vec: Vec<ListItem> = Vec::new();

    let mut index = start_index;
    loop {
        let token_wrap = tokens.get(index);
        if (token_wrap.is_none()) {
            break;
        }
        let token = token_wrap.unwrap();
        match token {
            Token::StartOfArr => {
                let (next_index, list) = parse_tokens(index + 1, tokens);
                vec.push(list);
                index = next_index;
            }
            Token::EndOfArr => {
                break;
            }
            Token::Num(x) => vec.push(ListItem::Num(*x)),
            Token::Dot => {}
        }

        index += 1;
    }

    (index, ListItem::List(vec))
}

pub fn peek(str: &str) -> (usize, Token) {
    let mut s = str.chars().collect::<Vec<_>>();
    let mut num = String::new();
    let mut is_num = false;

    let first = s.get(0).unwrap();

    for c in str.chars() {
        if is_num == false {
            let token = Token::new(c);
            match token {
                Token::Num(x) => {
                    is_num = true;
                    num = format!("{}", x)
                }
                _ => return (1, token),
            }
            continue;
        }
        let token = Token::new(c);
        match token {
            Token::Num(n) => num = format!("{}{}", n, num),
            _ => break,
        }
    }

    (num.len(), Token::Num(num.parse::<i32>().unwrap()))
}
