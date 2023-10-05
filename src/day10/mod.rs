#![allow(unused)]

use crate::utils::read_file;

#[derive(Debug)]
enum Cmd {
    Loop,
    Add(i32),
}

// https://adventofcode.com/2022/day/10
pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let signals = parse_input();
    // println!("{:?}", signals);
    let mut marks_signals: Vec<i32> = Vec::new();
    let marks = [20, 60, 100, 140, 180, 220];
    for mark in marks {
        let find = get_cur_pos(mark, &signals);
        match find {
            Some(t) => {
                marks_signals.push(mark * t.1);
            }
            _ => {}
        }
    }

    println!(
        "{:?} {:?}",
        marks_signals,
        marks_signals.iter().sum::<i32>()
    );
}

fn parse2() {
    let signals = parse_input();
    // println!("{:?}", signals);
    let mut arr: Vec<Vec<&str>> = Vec::new();
    for i in 0..=5 {
        let mut list: Vec<&str> = Vec::new();
        for j in 1..=40 {
            let cur_index = j + i * 40;
            let cur_signal = get_cur_pos(cur_index, &signals);
            let cur_mark_pos = match cur_signal {
                Some(t) => t.1,
                _ => 1,
            };

            let show_mark = j - cur_mark_pos <= 2 && j - cur_mark_pos >= 0;
            if show_mark {
                list.push("#");
            } else {
                list.push(".");
            }
            if cur_index == 10 {
                println!(
                    "cur_index={:?} cur_mark_pos={:?} match={:?}",
                    cur_index,
                    cur_mark_pos,
                    (j - cur_mark_pos).abs() < 2
                )
            }
        }
        arr.push(list);
    }

    // RFKZCPEF
    for line in arr.iter() {
        println!("{:?}", line.join(""));
    }
}

fn get_cur_pos(
    cur_index: i32,
    signals: &Vec<(i32, i32)>,
) -> Option<(i32, i32)> {
    let mut find = signals.iter().find(|(cycle, num)| cycle == &cur_index);
    if find.is_none() {
        find = signals
            .iter()
            .find(|(cycle, num)| cycle.clone() == &cur_index - 1);
    }

    match find {
        Some(t) => Some((t.0.clone(), t.1.clone())),
        _ => None,
    }
}

fn parse_input() -> Vec<(i32, i32)> {
    let content = read_file("day10/input.txt").unwrap();

    let arr = content
        .split("\n")
        .map(|line| {
            if line == "noop" {
                return Cmd::Loop;
            }
            let arr = line.split(" ").collect::<Vec<_>>();
            let num = arr[1].parse::<i32>().unwrap();
            return Cmd::Add(num);
        })
        .collect::<Vec<_>>();

    let mut cur_cycle = 1;
    let mut cur_num = 1;
    let mut signals: Vec<(i32, i32)> = Vec::new();
    for (index, item) in arr.iter().enumerate() {
        match item {
            Cmd::Loop => cur_cycle += 1,
            Cmd::Add(num) => {
                cur_cycle += 2;
                cur_num += num;
            }
        }
        signals.push((cur_cycle, cur_num));
    }

    signals
}
