use super::{has_pass_path, parse_input, PathKey};

// static TEST1: &str  = "AA-false|II-false|JJ-true|II-false|AA-false|BB-true|CC-false|DD-true|EE-true|FF-false|GG-false|HH-true";
// static TEST1: &str  = "AA-false|DD-true|CC-false|BB-true|AA-false|II-false|JJ-true|II-false|AA-false|DD-false|EE-false|FF-false|GG-false|HH-true|GG-false|FF-false|EE-true|DD-false|CC-true";
static TEST1: &str  = "AA-false|DD-true|CC-false|BB-true|AA-false|II-false|JJ-true|II-false|AA-false|DD-false|EE-true|FF-false|GG-false|HH-true";
static TEST2: [(&str, bool); 14] = [
    ("AA", false),
    ("DD", true),
    ("CC", true),
    ("BB", true),
    ("AA", false),
    ("II", false),
    ("JJ", true),
    ("II", false),
    ("AA", false),
    ("DD", false),
    ("EE", true),
    ("FF", false),
    ("GG", false),
    ("HH", true),
];

pub fn test_path_score(path_arr: PathKey) {
    let map = parse_input();
    // let path_arr = str_to_path(TEST1);
    // let path_arr = str_arr_to_path(TEST2.to_vec());

    let mut pass_path = String::from("");
    let mut cur_time = 30 as i32;
    let mut score = 0 as usize;
    for item in path_arr.iter() {
        if cur_time <= 0 {
            break;
        }
        let (name, is_open) = item;
        let mut value = map.get_value(name).unwrap();
        if *is_open {
            cur_time -= 1;
            score += value.rate as usize * cur_time as usize;
        }

        cur_time -= 1;
        // pass_path = format!("{}-{}", pass_path, item.n);
    }

    println!("score={:?}", score)
}

pub fn test_pass_path() {
    let path_arr = str_to_path(
        "AA-false|DD-true|AA-false|DD-false|CC-true|BB-true|CC-false|DD-false",
    );

    let res = has_pass_path(
        [(String::from("CC"), false), (String::from("DD"), false)],
        &path_arr,
    );

    println!("{}", res);
}

pub fn str_arr_to_path(raw_path: Vec<(&str, bool)>) -> PathKey {
    return raw_path
        .into_iter()
        .map(|arr| {
            return (String::from(arr.0), arr.1);
        })
        .collect::<Vec<_>>();
}

pub fn str_to_path(s: &str) -> PathKey {
    return s
        .split("|")
        .map(|item| {
            let arr = item.split("-").collect::<Vec<_>>();

            return (String::from(arr[0]), arr[1] == "true");
        })
        .collect::<Vec<_>>();
}

// fn test1() {
//     let map = parse_input();
//     // let path = "AA-DD-AA-BB-AA-II-JJ-II-AA-DD-EE-FF-GG-HH-GG-FF-EE-DD-CC";
//     let path = "AA-DD-CC-BB-AA-II-JJ-II-AA-DD-EE-FF-GG-HH-GG-FF-EE-DD-CC";
//     let path_arr = path.split("-").collect::<Vec<_>>();

//     let mut pass_path = String::from("");
//     let mut cur_time = 30;
//     let mut score = 0 as usize;
//     for name in path_arr.iter() {
//         let mut value = map.get_value(name).unwrap();

//         if value.rate > 0 && value.is_open == false {
//             cur_time -= 1;
//             score += value.rate as usize * cur_time;
//             value.set_open(true)
//         }
//         println!(
//             "item={:?}| cur_time={:?} | rate={} | cur_score={:?}",
//             name, cur_time, value.rate, score
//         );
//         cur_time -= 1;
//         pass_path = format!("{}-{}", pass_path, name);
//     }

//     println!("score={:?}\npass_path={:?}", score, pass_path)
// }
