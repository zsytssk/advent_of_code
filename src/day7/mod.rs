#![allow(unused)]

use std::{cell::RefCell, rc::Rc};

use crate::utils::read_file;
use once_cell::sync::Lazy;
use regex::Regex;

mod node;

use node::*;

enum Cmd {
    Cd(String),
    Ls,
}

// resolve path
// node.rs 使用起来非常麻烦，有什么办法来优化
// 这里面最大的问题是 每一步操作都要去做判断 NodeType，还有就是NodeWrap和node的转化
// https://adventofcode.com/2022/day/7
pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let s = parse_input();
    let lines: Vec<&str> = s.split("\n").collect();
    let root = execute(lines);

    let mut sub_dir = Dir::get_sub_dir(&root);
    sub_dir.insert(0, root);

    let size_vec: u32 = sub_dir
        .iter()
        .map(|item| Dir::get_wrap_size(item))
        .filter(|&i| i < 100000)
        .sum();

    println!("size_all = {:?}", size_vec);
}

fn parse2() {
    let s = parse_input();
    let lines: Vec<&str> = s.split("\n").collect();
    let root = execute(lines);

    let mut sub_dir = Dir::get_sub_dir(&root);
    sub_dir.insert(0, root);

    let mut size_vec = sub_dir
        .iter()
        .map(|item| (Dir::get_wrap_name(item), Dir::get_wrap_size(item)))
        .collect::<Vec<(String, u32)>>();

    size_vec.sort_by(|a, b| a.1.cmp(&b.1));

    let total_size = 30000000 - (70000000 - size_vec.last().unwrap().1);

    for item in size_vec.iter() {
        if item.1 > total_size {
            println!("{} = {}", item.0, item.1);
            break;
        }
    }

    // println!("size:list = {:?}", size_vec);
}

fn execute(mut lines: Vec<&str>) -> NodeWrap {
    let mut root = Dir::new_wrap("root");

    let mut cur_dir = Rc::clone(&root);
    for (index, line) in lines.iter().enumerate() {
        if line.contains('$') == false {
            continue;
        }
        let cmd = parse_line(line);
        match cmd {
            Cmd::Cd(s) => {
                if s == "/" {
                    cur_dir = Rc::clone(&root);
                } else if s == ".." {
                    cur_dir = Dir::get_wrap_parent(&cur_dir)
                        .expect(format!("cant find parent {}", s).as_str());
                } else {
                    cur_dir = Dir::find_child_dir(&cur_dir, &s)
                        .expect(format!("cant find dir {}", s).as_str());
                }
            }
            Cmd::Ls => {
                let ls_con = get_ls_content(index, &lines);
                for child in ls_con {
                    Dir::add_child(&mut cur_dir, child);
                }
            }
        }
    }

    root
}

fn parse_line(line: &str) -> Cmd {
    let cd_rex: Regex = Regex::new(r"\$ cd ([^ ]+)").unwrap();
    let ls_rex: Regex = Regex::new(r"\$ ls").unwrap();

    match cd_rex.captures(line) {
        Some(t) => {
            let s = (t.get(1).unwrap().as_str());
            return Cmd::Cd(String::from(s));
        }
        None => {}
    }

    match ls_rex.captures(line) {
        Some(t) => return Cmd::Ls,
        None => {
            panic!("cant find match cmd for: {}", line)
        }
    }
}

fn get_ls_content(index: usize, lines: &Vec<&str>) -> Vec<NodeType> {
    let mut content: Vec<String> = Vec::new();
    for i in index + 1..lines.len() {
        let line = lines.get(i).unwrap();
        if line.contains('$') {
            break;
        }
        content.push(String::from(*line));
    }

    content.iter().map(|item| parse_ls_item(item)).collect()
}

fn parse_ls_item(line: &str) -> NodeType {
    let dir_rex: Regex = Regex::new(r"dir (\w+)").unwrap();
    let file_rex: Regex = Regex::new(r"(\d+) (\w+(\.\w+)*)").unwrap();

    match dir_rex.captures(line) {
        Some(t) => {
            let name = t.get(1).unwrap().as_str();
            return NodeType::Dir(Dir::new(name));
        }
        None => {}
    }
    match file_rex.captures(line) {
        Some(t) => {
            let size = t.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let name = t.get(2).unwrap().as_str();
            NodeType::File(File::new(name, size))
        }
        None => panic!("cant find ls type for {}", line),
    }
}

fn parse_input() -> String {
    let content = read_file("day7/input.txt").unwrap();

    content
}
