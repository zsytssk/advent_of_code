use crate::utils::read_file;
use regex::Regex;

pub fn parse() {
    parse1();

    parse2_1();
}
fn parse2_1() {
    let content = read_file("day1/input2.txt").unwrap();

    let mut arr: Vec<i32> = content
        .split("\n\n")
        .map(|x| x.split("\n").map(|x| x.parse::<i32>().unwrap()).sum())
        .collect();

    arr.sort();

    let len = arr.len();
    let a: i32 = arr[len - 3..len].iter().sum();

    println!("parse1: {:?}", a);
}

fn parse1() {
    let content = read_file("day1/input.txt").unwrap();
    let multi_line_reg = Regex::new(r"\n{2,}").unwrap();
    let line_reg = Regex::new(r"\n").unwrap();

    let arr: i32 = multi_line_reg
        .split(&content)
        .map(|line| {
            line_reg
                .split(line)
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .sum()
        })
        .max()
        .unwrap();

    println!("parse2: {:?}", arr);
}
