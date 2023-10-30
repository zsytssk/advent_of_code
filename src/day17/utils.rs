use super::rock::{Dot, RockShape};

/** 通过类型创建石头的点 */
pub fn create_dots(shape: &RockShape) -> Vec<Dot> {
    match shape {
        RockShape::LineFour => {
            vec![
                Dot::new(0, 0),
                Dot::new(1, 0),
                Dot::new(2, 0),
                Dot::new(3, 0),
            ]
        }
        RockShape::CrossLine => {
            vec![
                Dot::new(1, 0),
                Dot::new(0, 1),
                Dot::new(1, 1),
                Dot::new(2, 1),
                Dot::new(1, 2),
            ]
        }
        RockShape::LShape => {
            vec![
                Dot::new(0, 0),
                Dot::new(1, 0),
                Dot::new(2, 0),
                Dot::new(2, 1),
                Dot::new(2, 2),
            ]
        }
        RockShape::Vertical => {
            vec![
                Dot::new(0, 0),
                Dot::new(0, 1),
                Dot::new(0, 2),
                Dot::new(0, 3),
            ]
        }
        RockShape::Square => {
            vec![
                Dot::new(0, 0),
                Dot::new(1, 0),
                Dot::new(0, 1),
                Dot::new(1, 1),
            ]
        }
    }
}
// pub fn create_dots(shape: &RockShape) -> Vec<Dot> {
//     match shape {
//         RockShape::LineFour => {
//             vec![
//                 Dot::new(0, 0),
//                 Dot::new(1, 0),
//                 Dot::new(2, 0),
//                 Dot::new(3, 0),
//             ]
//         }
//         RockShape::CrossLine => {
//             vec![
//                 Dot::new(1, 0),
//                 Dot::new(0, 1),
//                 Dot::new(1, 1),
//                 Dot::new(2, 1),
//                 Dot::new(1, 2),
//             ]
//         }
//         RockShape::LShape => {
//             vec![
//                 Dot::new(2, 0),
//                 Dot::new(2, 1),
//                 Dot::new(0, 2),
//                 Dot::new(1, 2),
//                 Dot::new(2, 2),
//             ]
//         }
//         RockShape::Vertical => {
//             vec![
//                 Dot::new(0, 0),
//                 Dot::new(0, 1),
//                 Dot::new(0, 2),
//                 Dot::new(0, 3),
//             ]
//         }
//         RockShape::Square => {
//             vec![
//                 Dot::new(0, 0),
//                 Dot::new(1, 0),
//                 Dot::new(0, 1),
//                 Dot::new(1, 1),
//             ]
//         }
//     }
// }
