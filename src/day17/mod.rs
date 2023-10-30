#![allow(unused)]
use std::time::Instant;

use crate::utils::read_file;

mod rock;
mod utils;

use rock::*;

pub fn parse() {
    // parse1();
    parse2();
}

const ROCK_ORDER: [RockShape; 5] = [
    RockShape::LineFour,
    RockShape::CrossLine,
    RockShape::LShape,
    RockShape::Vertical,
    RockShape::Square,
];

fn parse1() {
    let now = Instant::now();
    let content = parse_input();
    let mut chamber = Chamber::new(7, 0);

    let mut char_num = 0;
    for i in 0..2022 {
        let type_index = i % 5;
        let shape = ROCK_ORDER[type_index].clone();
        let mut rock = Rock::new(shape, 0, 0);
        chamber.adjust_rock(&mut rock);

        loop {
            let char_index = char_num % content.len();
            let move_char =
                content.chars().nth(char_index % content.len()).unwrap();
            char_num += 1;

            let dir = Dir::from_str(&move_char);
            let res = chamber.move_rock_to(&mut rock, dir);

            let moved = chamber.move_rock_to(&mut rock, Dir::Down);
            if moved == false {
                chamber.add_rock(rock);
                break;
            }
        }
    }

    // println!("{}", chamber.get_fmt_str());
    println!(
        "time={:?} | height={} | top_height={}",
        now.elapsed(),
        chamber.rock_list.len(),
        chamber.top_height
    );
}

fn parse2() {
    let now = Instant::now();
    let content = parse_input();
    let mut chamber = Chamber::new(7, 0);

    let mut char_num = 0;
    // for i in 0..1000000000000 {
    for i in 0..100000 {
        let type_index = i % 5;
        let shape = ROCK_ORDER[type_index].clone();
        let mut rock = Rock::new(shape, 0, 0);
        chamber.adjust_rock(&mut rock);

        loop {
            let char_index = char_num % content.len();
            let move_char =
                content.chars().nth(char_index % content.len()).unwrap();
            char_num += 1;

            let dir = Dir::from_str(&move_char);
            let res = chamber.move_rock_to(&mut rock, dir);

            let moved = chamber.move_rock_to(&mut rock, Dir::Down);
            if moved == false {
                chamber.add_rock(rock);
                break;
            }
        }
    }

    // println!("{}", chamber.get_fmt_str());
    println!(
        "time={:?} | height={}",
        now.elapsed(),
        chamber.get_top_height()
    );
}

fn parse_input() -> String {
    let content = read_file("day17/demo.txt").unwrap();

    content
}

#[cfg(test)]
mod tests {
    use super::{
        rock::{Chamber, Dir, Rock, RockShape},
        ROCK_ORDER,
    };

    #[test]
    fn print_rock1() {
        let mut chamber = Chamber::new(7, 0);
        let mut rock = Rock::new(ROCK_ORDER[3].clone(), 0, 0);
        chamber.adjust_rock(&mut rock);

        chamber.move_rock_to(&mut rock, Dir::Right);
        chamber.move_rock_to(&mut rock, Dir::Down);
        chamber.move_rock_to(&mut rock, Dir::Right);
        chamber.move_rock_to(&mut rock, Dir::Down);
        chamber.move_rock_to(&mut rock, Dir::Left);
        chamber.move_rock_to(&mut rock, Dir::Down);
        chamber.move_rock_to(&mut rock, Dir::Right);
        chamber.add_rock(rock);
        println!("{}", chamber.get_fmt_str());
    }
    #[test]
    fn print_rock2() {
        for item in ROCK_ORDER.iter() {
            let rock = Rock::new(item.clone(), 0, 0);
            println!("{:?}", rock.get_inner_range());
        }
    }

    #[test]
    fn print_champer() {
        let mut chamber = Chamber::new(7, 0);

        for i in 0..5 {
            let type_index = i % 5;
            let rock = Rock::new(ROCK_ORDER[type_index].clone(), 0, 0);
            chamber.add_rock(rock);
        }

        // println!("{:?}", chamber);
        println!("{}", chamber.get_fmt_str());
    }
}
