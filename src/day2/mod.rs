#![allow(unused)]
use crate::utils::read_file;

#[derive(Debug)]
enum GameType {
  Rock,
  Paper,
  Scissors,
}

#[derive(Debug)]
enum MatchType {
  Lost,
  Draw,
  Won,
}

pub fn parse() {
  parse1();
  parse2();
}

fn parse1() {
  let content = read_file("day2/input.txt").unwrap();
  let result: u32 = content
    .split("\n")
    .map(|item| {
      let line_split: Vec<&str> = item.split(" ").collect();
      let op = get_type_from_str(line_split[0]);
      let my = get_type_from_str(line_split[1]);
      let type_score = get_score_from_type(&my);
      let match_result = get_match_result(&my, &op);
      let match_score = get_score_from_match(&match_result);
      let res = type_score + match_score;
      res as u32
    })
    .sum();

  println!("round1 {:?}", result);
}

fn parse2() {
  let content = read_file("day2/input.txt").unwrap();
  let result: u32 = content
    .split("\n")
    .map(|item| {
      let line_split: Vec<&str> = item.split(" ").collect();
      let op = get_type_from_str(line_split[0]);
      let match_result = get_result_from_str(line_split[1]);
      let my = get_my_score_from_rest(&op, &match_result);
      let type_score = get_score_from_type(&my);
      let match_score = get_score_from_match(&match_result);
      let res = type_score + match_score;
      // println!(
      //     "op={:?} my={:?} match={:?} res={:?}",
      //     op, my, match_result, res
      // );
      res as u32
    })
    .sum();

  println!("round1 {:?}", result);
}

fn get_type_from_str(my_str: &str) -> GameType {
  match my_str {
    "A" | "X" => GameType::Rock,
    "B" | "Y" => GameType::Paper,
    "C" | "Z" => GameType::Scissors,
    _ => panic!("cant find game type for {}", my_str),
  }
}

fn get_result_from_str(my_str: &str) -> MatchType {
  match my_str {
    "X" => MatchType::Lost,
    "Y" => MatchType::Draw,
    "Z" => MatchType::Won,
    _ => panic!("cant find result type for {}", my_str),
  }
}

fn get_score_from_type(game_type: &GameType) -> u8 {
  match game_type {
    GameType::Rock => 1,
    GameType::Paper => 2,
    GameType::Scissors => 3,
  }
}

fn get_score_from_match(match_type: &MatchType) -> u8 {
  match match_type {
    MatchType::Lost => 0,
    MatchType::Draw => 3,
    MatchType::Won => 6,
  }
}

fn get_match_result(my: &GameType, op: &GameType) -> MatchType {
  match (my, op) {
    (GameType::Rock, GameType::Rock)
    | (GameType::Paper, GameType::Paper)
    | (GameType::Scissors, GameType::Scissors) => MatchType::Draw,
    (GameType::Rock, GameType::Scissors)
    | (GameType::Scissors, GameType::Paper)
    | (GameType::Paper, GameType::Rock) => MatchType::Won,
    (GameType::Rock, GameType::Paper)
    | (GameType::Paper, GameType::Scissors)
    | (GameType::Scissors, GameType::Rock) => MatchType::Lost,
  }
}

fn get_my_score_from_rest(op: &GameType, res: &MatchType) -> GameType {
  match (op, res) {
    (GameType::Rock, MatchType::Draw)
    | (GameType::Paper, MatchType::Lost)
    | (GameType::Scissors, MatchType::Won) => GameType::Rock,
    (GameType::Rock, MatchType::Won)
    | (GameType::Scissors, MatchType::Lost)
    | (GameType::Paper, MatchType::Draw) => GameType::Paper,
    (GameType::Rock, MatchType::Lost)
    | (GameType::Paper, MatchType::Won)
    | (GameType::Scissors, MatchType::Draw) => GameType::Scissors,
  }
}
