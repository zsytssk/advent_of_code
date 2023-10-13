#[derive(Debug)]
pub struct Value {
    name: String,
    rate: u8,
    to: Vec<String>,
}

impl Value {
    pub fn from_str(s: &str) -> Self {
        let regex = regex::Regex::new(
            r"Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (\w+[, \w+]*)",
        )
        .unwrap();
        let m = regex.captures(s).unwrap();
        let (_, match_list): (&str, [&str; 3]) = m.extract();

        Value {
            name: match_list[0].to_string(),
            rate: match_list[1].parse().unwrap(),
            to: match_list[2].split(", ").map(|s| s.to_string()).collect(),
        }
    }
}
