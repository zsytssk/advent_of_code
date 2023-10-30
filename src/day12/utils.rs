use std::cell::Ref;

use super::map::Point;

pub fn calc_top_path<'a>(
    loop_paths: &mut Vec<(Ref<'a, Point>, i32, i32)>,
    end: Ref<Point>,
) -> Vec<(Ref<'a, Point>, i32, i32)> {
    loop_paths.sort_by(|a, b| {
        let a_move = a.1;
        let b_move = b.1;
        let a_dis = end.distance(&a.0);
        let b_dis = end.distance(&b.0);

        (a.2).cmp(&b.2)
    });

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
