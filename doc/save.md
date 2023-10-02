- 为什么我把第一行的注释打开这个代码就报错了

```rs
// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum NodeType {
    File(u32),
    Dir(u32),
}

fn main() {
    let node = Rc::new(RefCell::new(NodeType::File(42)));

    {
        let b = &node;
        let a = &mut *b.borrow_mut();
        match a {
            NodeType::File(file) => {
                // Handle `NodeType::File` variant
                *file += 1;
                println!("File: {}", file);
            }
            NodeType::Dir(dir) => {
                // Handle `NodeType::Dir` variant
                *dir += 1;
                println!("Dir: {}", dir);
            }
        }
    }
}

```
