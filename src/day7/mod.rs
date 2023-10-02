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
// get_node_by_path
// https://adventofcode.com/2022/day/7
pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let s = parse_input();
    let lines: Vec<&str> = s.split("\n").collect();
    let root = execute(lines);
    println!("root = {:?}", root);
}

fn parse2() {}

fn execute(mut lines: Vec<&str>) -> DirWrap {
    let mut root = Dir::new_rf("root");

    let mut cur_dir = Rc::clone(&root);
    for (index, line) in lines.iter().enumerate() {
        if line.contains('$') == false {
            continue;
        }
        let cmd = parse_line(line);
        match cmd {
            Cmd::Cd(s) => {
                println!("cd {}", s);
                if s == "/" {
                    cur_dir = Rc::clone(&root);
                } else if s == ".." {
                    let cur_dir = Rc::clone(&cur_dir);
                } else {
                }
            }
            Cmd::Ls => {
                let ls_con = get_ls_content(index, &lines);
                println!("ls_con {:?}", ls_con);
                for child in ls_con {
                    Dir::add_child(&cur_dir, child);
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
    let content = read_file("day7/demo.txt").unwrap();

    content
}
