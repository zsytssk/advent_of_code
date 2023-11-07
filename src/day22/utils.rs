use regex::Regex;

pub fn split_keep(re: Regex, text: &str) -> Vec<String> {
  let mut prev_end = 0;

  let mut arr = vec![];
  for mat in re.find_iter(text) {
    let start = mat.start();
    let end = mat.end();

    arr.push(text[prev_end..start].to_owned());
    arr.push(text[start..end].to_owned());

    prev_end = end;
  }

  if text[prev_end..].len() > 0 {
    arr.push(text[prev_end..].to_owned());
  }

  arr
}
