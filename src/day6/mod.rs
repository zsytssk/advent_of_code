#![allow(unused)]
use std::vec::Drain;

use crate::utils::read_file;
use regex::Regex;

// 取最后一行的字母，然后用他的index取其他行的内容
// https://adventofcode.com/2022/day/6#part2
pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let str = parse_input();

    let mut marker_pos = 0;
    let mut marker = String::from("");
    for (index, ch) in str.chars().enumerate() {
        marker.push(ch);
        if marker.len() < 4 {
            continue;
        }
        if marker.len() > 4 {
            marker.remove(0);
        }
        if is_marker(&marker) {
            marker_pos = index;
            break;
        }
    }

    println!("marker pos {}", marker_pos + 1);
}

fn parse2() {
    let str = parse_input();

    let mut marker_pos = 0;
    let mut marker = String::from("");
    for (index, ch) in str.chars().enumerate() {
        marker.push(ch);
        if marker.len() < 14 {
            continue;
        }
        if marker.len() > 14 {
            marker.remove(0);
        }
        if is_marker(&marker) {
            marker_pos = index;
            break;
        }
    }

    println!("marker pos {}", marker_pos + 1);
}

fn parse_input() -> String {
    let content = read_file("day6/demo.txt").unwrap();

    content
}

fn is_marker(str: &str) -> bool {
    for (index, ch) in str.chars().enumerate() {
        let r_index = str.rfind(ch).unwrap();
        if r_index != index {
            return false;
        }
    }

    true
}
