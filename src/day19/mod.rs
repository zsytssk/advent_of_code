#![allow(unused)]
use regex::Regex;

use crate::utils::read_file;

use self::blueprint::{Blueprint, CostType, Robot};

mod blueprint;
use blueprint::*;

pub fn parse() {
    parse1();
    // parse2();
}

fn parse1() {
    let content = parse_input1();
    println!("{:?}", content);

    let mut save_list: Vec<LoopInfo> = vec![];
    let mut loop_list: Vec<LoopInfo> = vec![];
    loop {
        for item in loop_list.iter() {}
    }
}

fn parse2() {
    let content = parse_input2();
    println!("{:?}", content.len());
}

fn parse_input1() -> Vec<Blueprint> {
    let title_reg = Regex::new(r"Blueprint (\d+):").unwrap();
    let robot_reg = Regex::new(
        r"Each (\w+) robot costs (\d+) ore( and (\d+) (clay|obsidian)+)?\.",
    )
    .unwrap();
    let content = read_file("day19/demo.txt").unwrap();

    content
        .split("\n\n")
        .map(|block| {
            let lines = block.split("\n").collect::<Vec<_>>();
            let title_find = title_reg.captures(lines[0]);
            let name = title_find
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();

            let mut robots = Vec::new();
            for i in 1..lines.len() {
                let line_find = robot_reg.captures(lines[i]).unwrap();
                let name: &str = line_find.get(1).unwrap().as_str();
                let cost_ore: i32 =
                    line_find.get(2).unwrap().as_str().parse().unwrap();
                let mut cost_arr = vec![(cost_ore, CostType::Ore)];

                match line_find.get(5) {
                    None => {}
                    Some(t) => {
                        let extra_name = CostType::form_str(t.as_str());
                        let extra_num: i32 =
                            line_find.get(4).unwrap().as_str().parse().unwrap();

                        cost_arr.push((extra_num, extra_name));
                    }
                };

                let robot = Robot::new(name, cost_arr);
                robots.push(robot);
            }

            Blueprint::new(name, robots)
        })
        .collect::<Vec<_>>()
}

fn parse_input2() -> Vec<Blueprint> {
    let title_reg = Regex::new(r"Blueprint (\d+)").unwrap();
    let robot_reg = Regex::new(
        r"Each (\w+) robot costs (\d+) ore( and (\d+) (clay|obsidian)+)?",
    )
    .unwrap();
    let content = read_file("day19/input.txt").unwrap();

    content
        .split("\n")
        .map(|block| {
            let lines = block.split(":").collect::<Vec<_>>();
            let title_find = title_reg.captures(lines[0]);
            let name = title_find
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();

            let line_list = lines[1]
                .split(".")
                .filter(|item| item.len() > 0)
                .collect::<Vec<_>>();

            let mut robots = Vec::new();
            for line in line_list {
                let line_find = robot_reg.captures(line).unwrap();
                let name: &str = line_find.get(1).unwrap().as_str();
                let cost_ore: i32 =
                    line_find.get(2).unwrap().as_str().parse().unwrap();
                let mut cost_arr = vec![(cost_ore, CostType::Ore)];

                match line_find.get(5) {
                    None => {}
                    Some(t) => {
                        let extra_name = CostType::form_str(t.as_str());
                        let extra_num: i32 =
                            line_find.get(4).unwrap().as_str().parse().unwrap();

                        cost_arr.push((extra_num, extra_name));
                    }
                };

                let robot = Robot::new(name, cost_arr);
                robots.push(robot);
            }

            Blueprint::new(name, robots)
        })
        .collect()
}
