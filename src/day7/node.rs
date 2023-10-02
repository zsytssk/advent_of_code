use core::panic;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum NodeType {
    File(File),
    Dir(Dir),
}

pub type NodeWrap = Rc<RefCell<NodeType>>;
pub type ParentWrap = Weak<RefCell<NodeType>>;

#[derive(Debug)]
pub struct Dir {
    pub path: String,
    pub child: Vec<NodeWrap>,
    pub parent: Option<ParentWrap>,
}

impl Dir {
    pub fn new(path: &str) -> Self {
        Dir {
            path: String::from(path),
            child: Vec::new(),
            parent: None,
        }
    }
    pub fn new_wrap(path: &str) -> NodeWrap {
        Rc::new(RefCell::new(NodeType::Dir(Dir {
            path: String::from(path),
            child: Vec::new(),
            parent: None,
        })))
    }
    pub fn find_child_dir(parent: &NodeWrap, name: &str) -> Option<NodeWrap> {
        let NodeType::Dir(dir) = &*parent.borrow() else {
            panic!("not a dir");
        };

        for item in dir.child.iter() {
            match &*item.borrow() {
                NodeType::Dir(dir) => {
                    if dir.path == name {
                        return Some(Rc::clone(item));
                    }
                }
                _ => continue,
            }
        }

        None
    }
    pub fn get_sub_dir(parent: &NodeWrap) -> Vec<NodeWrap> {
        let mut sub_dir: Vec<NodeWrap> = Vec::new();

        let NodeType::Dir(dir) =  &*parent.borrow() else {
            panic!("not a dir");
        };

        for item in dir.child.iter() {
            match &*item.borrow() {
                NodeType::Dir(dir) => {
                    let inner_dir = Dir::get_sub_dir(item);
                    sub_dir.extend(inner_dir);
                    sub_dir.push(Rc::clone(item));
                }
                _ => continue,
            }
        }

        sub_dir
    }
    pub fn set_parent(&mut self, dir: ParentWrap) {
        self.parent = Some(dir);
    }
    pub fn add_child(parent: &NodeWrap, mut child: NodeType) {
        match &mut child {
            NodeType::File(file) => {
                file.set_parent(Rc::downgrade(parent));
            }
            NodeType::Dir(dir) => {
                dir.set_parent(Rc::downgrade(parent));
            }
        }
        match &mut *parent.borrow_mut() {
            NodeType::Dir(dir) => {
                dir.child.push(Rc::new(RefCell::new(child)));
            }
            _ => panic!("not a dir"),
        }
    }
    pub fn get_wrap_parent(wrap: &NodeWrap) -> Option<NodeWrap> {
        match &*wrap.borrow() {
            NodeType::Dir(dir) => {
                return match &dir.parent {
                    Some(parent) => parent.upgrade(),
                    None => None,
                };
            }
            _ => None,
        }
    }
    pub fn get_wrap_size(wrap: &NodeWrap) -> u32 {
        let NodeType::Dir(dir) =  &*wrap.borrow() else {
            panic!("not a dir");
        };

        dir.get_size()
    }
    pub fn get_wrap_name(wrap: &NodeWrap) -> String {
        let NodeType::Dir(dir) =  &*wrap.borrow() else {
            panic!("not a dir");
        };

        String::from(&dir.path)
    }
    pub fn get_size(&self) -> u32 {
        let mut size = 0;

        for item in self.child.iter() {
            match &*item.borrow() {
                NodeType::File(file) => size = size + file.size,
                NodeType::Dir(dir) => size = size + dir.get_size(),
            }
        }

        return size;
    }
}

#[derive(Debug)]
pub struct File {
    size: u32,
    name: String,
    parent: Option<ParentWrap>,
}

impl File {
    pub fn set_parent(&mut self, dir: ParentWrap) {
        self.parent = Some(dir);
    }
    pub fn new(name: &str, size: u32) -> Self {
        File {
            name: String::from(name),
            size,
            parent: None,
        }
    }
}
