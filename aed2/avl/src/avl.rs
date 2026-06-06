#![allow(dead_code)]
#![allow(unused)]

use std::ptr::NonNull;

use crate::node::Node;

pub struct AVLTree<T, U> {
    root: Option<NonNull<Node<T, U>>>,
}

impl<T: Clone, U: Ord + Clone> AVLTree<T, U> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, data: T, key: U) {
        match self.root {
            Some(node) => {
                let new_node = Node::insert(Some(node), None, key, data);
                self.root = Some(new_node);
            }
            None => {
                let new_node = Node::insert(None, None, key, data);
                self.root = Some(new_node);
            }
        };
    }

    pub fn remove(&mut self, key: &U) {
        match self.root {
            Some(node) => {
                let new_node = Node::delete(Some(node), &key);
                self.root = new_node;
            }
            None => return,
        };
    }

    pub fn kth(&self, k: usize) -> Option<&Node<T, U>> {
        let ptr = Node::kth(self.root, k);

        return match ptr {
            Some(ptr) => unsafe { Some(ptr.as_ref()) },
            None => None,
        };
    }

    pub fn rank(&self, x: &U) -> usize {
        Node::rank(self.root, x)
    }
}
