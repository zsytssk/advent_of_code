#![allow(unused)]
use crate::utils::read_file;

pub fn parse() {
    parse1();
    // _parse2();
}

fn parse1() {
    let content = read_file("day3/input.txt").unwrap();
    let sum: u32 = content
        .split("\n")
        .map(|line| {
            let len = line.len();
            let first = &line[0..len / 2];
            let second = &line[len / 2..len];
            for c in first.chars() {
                if second.contains(c) {
                    return c;
                }
            }
            return ' ';
        })
        .map(|f| match f {
            'a'..='z' => f as u32 - 'a' as u32 + 1,
            'A'..='Z' => f as u32 - 'A' as u32 + 27,
            _ => 0,
        })
        .sum();

    println!("{:?}", sum);
}

// fn _parse2() {
//     let content = read_file("day2/input.txt").unwrap();
// }
