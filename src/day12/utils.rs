use std::{cell::Ref, collections::HashMap};

use super::{
    get_dir_pos,
    map::{Map, Point},
};

pub fn get_next_step<'a>(
    pos_wrap: &Ref<Point>,
    map: &'a Map,
) -> Vec<Ref<'a, Point>> {
    let pos = pos_wrap;

    let mut cur_arr = Vec::new();
    for dir in pos.get_move_dir().iter() {
        let (x, y) = get_dir_pos(pos_wrap, dir, map).unwrap();
        let next_pos = map.get_point(x, y).unwrap().borrow();
        cur_arr.push(next_pos);
    }

    cur_arr
}

pub fn calc_top_path<'a>(
    loop_paths: &mut Vec<(Ref<'a, Point>, i32, i32)>,
    end: Ref<Point>,
) -> Vec<(Ref<'a, Point>, i32, i32)> {
    loop_paths.sort_by(|a, b| (a.2).cmp(&b.2));

    let min_num = loop_paths[0].2;

    let mut remove_index = vec![];
    let mut arr = vec![];
    for (index, item) in loop_paths.iter().enumerate() {
        if item.2 == min_num {
            remove_index.push(index)
        }
    }
    for index in remove_index.into_iter().rev() {
        let item = loop_paths.remove(index);
        arr.push(item);
    }

    arr
}
