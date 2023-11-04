use std::{env, fs};

pub fn read_file(file: &str) -> Result<String, String> {
  let root = env::current_dir().unwrap();
  let file_path = root.join("src").join(file);

  fs::read_to_string(&file_path).map_err(|e| e.to_string())
}
