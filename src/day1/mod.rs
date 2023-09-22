use crate::utils::read_file;
use regex::Regex;

pub fn parse() {
    let content = read_file("day1/input.txt").unwrap();
    let multi_line_reg = Regex::new(r"\n{2,}").unwrap();
    let line_reg = Regex::new(r"\n").unwrap();
    // let arr: i32 = reg
    //     .split(&content)
    //     .map(|x| x.split("\n").map(|x| x.parse::<i32>().unwrap()).sum())
    //     .max()
    //     .unwrap();

    let arr: Vec<Vec<i32>> = multi_line_reg
        .split(&content)
        .map(|line| {
            line_reg
                .split(line)
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", arr);
}
// pub fn parse() {
//     let content = read_file("day1/input.txt").unwrap();

//     let arr: i32 = content
//         .split("\n\n")
//         .map(|x| x.split("\n").map(|x| x.parse::<i32>().unwrap()).sum())
//         .max()
//         .unwrap();

//     println!("{:?}", arr);
// }
