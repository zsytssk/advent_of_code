use std::{
    borrow::BorrowMut,
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum NodeType {
    File(File),
    Dir(Dir),
}

pub type ChildWrap = Rc<RefCell<NodeType>>;
pub type ParentWrap = Weak<RefCell<NodeType>>;

#[derive(Debug)]
pub struct Dir {
    pub path: String,
    pub child: Vec<ChildWrap>,
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
    pub fn new_rf(path: &str) -> ChildWrap {
        Rc::new(RefCell::new(NodeType::Dir(Dir {
            path: String::from(path),
            child: Vec::new(),
            parent: None,
        })))
    }
    pub fn find_child_dir(parent: &ChildWrap, name: &str) -> Option<ChildWrap> {
        let NodeType::Dir(dir) =  &*parent.borrow() else {
            panic!("not a dir");
        };

        for item in dir.child.iter() {
            match &*item.borrow() {
                NodeType::File(file) => continue,
                NodeType::Dir(dir) => {
                    if dir.path == name {
                        let a = item.borrow();
                        return Some(Rc::clone(item));
                    }
                }
            }
        }

        None
    }
    pub fn get_sub_dir(parent: &ChildWrap) -> Vec<ChildWrap> {
        let mut sub_dir: Vec<ChildWrap> = Vec::new();

        let NodeType::Dir(dir) =  &*parent.borrow() else {
            panic!("not a dir");
        };

        for item in dir.child.iter() {
            match &*item.borrow() {
                NodeType::File(file) => continue,
                NodeType::Dir(dir) => {
                    let inner_dir = Dir::get_sub_dir(item);
                    sub_dir.extend(inner_dir);
                    sub_dir.push(Rc::clone(item));
                }
            }
        }

        sub_dir
    }
    pub fn set_parent(&mut self, dir: ParentWrap) {
        self.parent = Some(dir);
    }
    pub fn add_child(parent: &mut ChildWrap, mut child: NodeType) {
        match &mut child {
            NodeType::File(file) => {
                file.set_parent(Rc::downgrade(parent));
            }
            NodeType::Dir(dir) => {
                dir.set_parent(Rc::downgrade(&parent));
            }
        }
        let mut a = &*parent.borrow();
        match &*parent.borrow() {
            NodeType::Dir(dir) => dir.child.push(Rc::new(RefCell::new(child))),
            _ => {}
        }
    }
    pub fn get_wrap_parent(wrap: &ChildWrap) -> Option<ChildWrap> {
        match &*wrap.borrow() {
            NodeType::Dir(dir) => {
                let parent = &dir.parent;
                Some((parent.unwrap().upgrade().unwrap()))
            }
            _ => None,
            NodeType::File(file) => None,
        }
    }
    pub fn get_wrap_size(wrap: &ChildWrap) -> u32 {
        let NodeType::Dir(dir) =  &*wrap.borrow() else {
            panic!("not a dir");
        };

        dir.get_size()
    }
    pub fn get_size(&self) -> u32 {
        let mut size = 0;

        println!("{:?}", &self.child);
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
