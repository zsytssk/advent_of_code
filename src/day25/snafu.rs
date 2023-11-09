use std::fmt;

use super::utils::parse_snafu;

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
  pub fn from_decimal(src: &str) -> Self {
    Snafu {
      src: src.to_string(),
      digits: Vec::new(),
    }
  }
  pub fn as_str(&self) -> &str {
    &self.src
  }
  pub fn to_decimal(&self) -> i64 {
    let digits = &self.digits;
    println!("digits:{:?}", digits);

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
