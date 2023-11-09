use super::snafu::Snafu;

pub fn parse_snafu(s: &str) -> Vec<i8> {
  s.split("")
    .filter(|&item| item != "")
    .map(|item| {
      if (item == "-") {
        return -1;
      }
      if (item == "=") {
        return -2;
      }
      return item.parse::<i8>().unwrap();
    })
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .collect()
}
