#![allow(unused)]

use std::{borrow::BorrowMut, cell::Cell};

use crate::utils::read_file;

mod monkey;

use monkey::*;
// https://adventofcode.com/2022/day/11
pub fn parse() {
    // parse1();
    parse2();
}

fn parse1() {
    let mut monkeys = parse_input();
    let len = monkeys.len();

    for round in 0..20 {
        for i in 0..len {
            let monkey = &mut monkeys[i];
            let res = monkey.run(true, 1);
            for (index, item) in res {
                monkeys[index as usize].add_num_list(item);
            }
        }
    }
    let mut inspected_items = monkeys
        .iter()
        .map(|m| m.get_inspected_items())
        .collect::<Vec<_>>();

    inspected_items.sort();
    let last1 = inspected_items[inspected_items.len() - 1];
    let last2 = inspected_items[inspected_items.len() - 2];
    println!("res {}", last1 * last2);
}

fn parse2() {
    let mut monkeys = parse_input();
    let len = monkeys.len();
    let multiplier = monkeys.iter().fold(1, |acc, monkey| acc * monkey.div);

    for round in 1..=10000 {
        for i in 0..len {
            let monkey = &mut monkeys[i];
            let res = monkey.run(false, multiplier);
            for (index, item) in res {
                monkeys[index as usize].add_num_list(item);
            }
        }
        let mut inspected_items = monkeys
            .iter()
            .map(|m| m.get_inspected_items())
            .collect::<Vec<_>>();

        println!("round={} inspected_items={:?}", round, monkeys[0].items);
    }
    let mut inspected_items = monkeys
        .iter()
        .map(|m| m.get_inspected_items())
        .collect::<Vec<_>>();

    inspected_items.sort();
    let last1 = inspected_items[inspected_items.len() - 1];
    let last2 = inspected_items[inspected_items.len() - 2];
    println!("res {}", last1 * last2);
}

fn parse_input() -> Vec<Monkey> {
    let content = read_file("day11/input.txt").unwrap();

    let ms = content
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Vec<_>>();

    return ms;
}
