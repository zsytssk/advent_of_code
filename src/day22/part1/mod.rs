use crate::utils::read_file;

use self::map::{Map, Path};

mod map;

use map::*;

pub fn parse() {
  let (mut map, path) = parse_input();

  let mut cur_pos = map.get_start();

  let mut i = 0;
  let mut path_arr: Vec<(usize, usize, Direction)> = vec![];
  // println!("path={:?}", path);

  loop {
    if i >= path.list.len() {
      break;
    }
    let cur_path = path.get_item(i);
    let next_pos = map.get_next_pos(cur_pos, cur_path, &mut path_arr);
    // println!(
    //   "loop={} | cur_pos={:?} | next_pos={:?} | cur_path={:?}",
    //   i, cur_pos, next_pos, cur_path
    // );
    cur_pos = next_pos;

    i += 1;
  }

  // println!(
  //   "cur_pos={:?} | dir={:?}",
  //   cur_pos,
  //   path.get_item(path.list.len() - 1)
  // );

  let num = cur_pos.y * 1000
    + cur_pos.x * 4
    + path.get_item(path.list.len() - 1).dir as usize;

  println!("num={:?}", num);

  // for p in path_arr.into_iter() {
  //   let point = map.get_mut_p(p.0, p.1);
  //   point.set_move_dir(p.2)
  // }
  // println!("{:?}", map);
}

fn parse_input() -> (Map, Path) {
  let content = read_file("day22/input.txt").unwrap();

  let mut blocks = content.split("\n\n").collect::<Vec<_>>();
  let map_str = blocks.remove(0);
  let path_str = blocks.remove(0);

  let map = Map::from_str(map_str);
  let path = Path::from_str(path_str);

  (map, path)
}
