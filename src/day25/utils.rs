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

pub fn init_to_snafu(num: f64) -> Vec<i8> {
  let neg_num = if num < 0.0 { -1.0 } else { 1.0 };
  let num = num.abs();

  let mut top_level = (num).log(5.0).floor();
  let cur_level_num = (5.0 as f64).powf(top_level);
  let mut top_num = (num / cur_level_num).floor();

  // 超过当前 top_level * 5，就要上升一位
  let mut upper_level = false;
  let upper_num = cur_level_num * 5.0;
  // println!("{:?}|{}", top_num, num > upper_num / 2.0);
  if num > upper_num / 2.0 {
    top_num = 1.0;
    top_level += 1.0;
    upper_level = true
  }

  let remain = if upper_level {
    (num - upper_num) * neg_num
  } else {
    (num - cur_level_num * top_num) * neg_num
  };

  let mut v = vec![0; (top_level as usize) + 1];
  v[top_level as usize] = (neg_num as i8) * top_num as i8;

  println!("{:?}|{}", v, remain);
  if remain != 0.0 {
    let sub_v = init_to_snafu(remain);
    for i in 0..sub_v.len() {
      v[i] += sub_v[i];
    }
  }

  v
}
pub fn to_int(arr: &Vec<i8>) -> i64 {
  let mut num = 0;
  for (index, i) in arr.iter().enumerate() {
    num += (*i as i64) * (5 as i64).pow(index as u32);
  }
  num
}

pub fn to_str(v: &Vec<i8>) -> String {
  v.iter()
    .rev()
    .map(|&x| {
      if x == -2 {
        return String::from("=");
      }
      if x == -1 {
        return String::from("-");
      }
      return x.to_string();
    })
    .collect::<Vec<_>>()
    .join("")
}
