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

    for item in items {
        match item {
            "[" => {}
            "]" => {}
            "," => {}
            x => {}
        }
    }
    println!("{:?}", str);
    ListItem::Num(0)
}
