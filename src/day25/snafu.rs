use std::fmt;

use super::utils::{init_to_snafu, parse_snafu, to_str};

pub struct Snafu {
  src: String,
  digits: Vec<i8>,
}

impl Snafu {
  pub fn new(src: &str) -> Self {
    let digits = parse_snafu(src);

    Snafu {
      src: src.to_string(),
      digits,
    }
  }
  pub fn from_decimal(num: i64) -> Self {
    let digits = init_to_snafu(num as f64);
    let src = to_str(&digits);

    Snafu { src, digits }
  }
  pub fn as_str(&self) -> &str {
    &self.src
  }
  pub fn to_decimal(&self) -> i64 {
    let digits = &self.digits;

    let mut num = 0;
    for (index, digit) in digits.iter().enumerate() {
      let base_num = (5 as i64).pow(index as u32) * (*digit as i64);
      num += base_num;
    }

    num
  }
}

impl fmt::Debug for Snafu {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.src)
  }
}
