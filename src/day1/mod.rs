use crate::utils::read_file;

pub fn parse() {
    let content = read_file("day1/input.txt").unwrap();
    println!("{content}");
}
