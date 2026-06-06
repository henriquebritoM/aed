#![allow(dead_code)]
#![allow(unused)]

pub struct AVLTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, value: T) -> bool {
        todo!()
    }

    pub fn remove(&mut self, value: T) -> bool {
        todo!()
    }

    pub fn contains(&self, value: T) -> bool {
        todo!()
    }

    pub fn empty(&self) -> bool {
        todo!()
    }
}
