use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum NodeType {
    File(File),
    Dir(Dir),
}

pub type DirWrap = Rc<RefCell<Dir>>;
pub type ChildWrap = Rc<RefCell<NodeType>>;

#[derive(Debug)]
pub struct Dir {
    pub path: String,
    pub child: Vec<ChildWrap>,
    pub parent: Option<DirWrap>,
}

impl Dir {
    pub fn new(path: &str) -> Self {
        Dir {
            path: String::from(path),
            child: Vec::new(),
            parent: None,
        }
    }
    pub fn new_rf(path: &str) -> DirWrap {
        Rc::new(RefCell::new(Dir {
            path: String::from(path),
            child: Vec::new(),
            parent: None,
        }))
    }
    pub fn find_child_dir(parent: &DirWrap, name: &str) -> Option<DirWrap> {
        for item in parent.borrow().child.iter() {
            match &*item.borrow() {
                NodeType::File(file) => continue,
                NodeType::Dir(dir) => {
                    if dir.path == name {
                        let a = item.borrow();
                        return Some(Rc::clone(&dir));
                    }
                }
            }
        }

        None
    }
    pub fn set_parent(&mut self, dir: DirWrap) {
        self.parent = Some(dir);
    }
    pub fn add_child(parent: &DirWrap, mut child: NodeType) {
        match &mut child {
            NodeType::File(file) => {
                file.set_parent(Rc::clone(&parent));
            }
            NodeType::Dir(dir) => {
                dir.set_parent(Rc::clone(&parent));
            }
        }
        parent.borrow_mut().child.push(Rc::new(RefCell::new(child)));
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
    parent: Option<DirWrap>,
}

impl File {
    pub fn set_parent(&mut self, dir: DirWrap) {
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
