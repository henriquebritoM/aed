#![allow(dead_code)]
#![allow(unused)]

/*
 *  The code here is beyond the touch of the creator.
 *
 *  The limited numbers of hours I could design to
 *  this project were not enough to fully adapt it
 *  to rust semantics and logic.
 *
 *  The result is this abomination, a pointer hell,
 *  tainted by the ugliests parts of rust and cpp,
 *  from where I copyed more logic than would be
 *  acceptable.
 *
 *  "abandon hope, all ye who enter here"
 */

use std::{
    cell::Cell,
    cmp::max,
    iter::Successors,
    ptr::{NonNull, null, null_mut},
    rc::Rc,
};

use crate::node;

pub struct Node<T, U> {
    key: U,
    data: T,
    height: usize,
    size: usize,
    balance: isize,
    parent: Option<NonNull<Node<T, U>>>,
    right: Option<NonNull<Node<T, U>>>,
    left: Option<NonNull<Node<T, U>>>,
}

// We need to manually free Nodes
impl<T, U> Drop for Node<T, U> {
    // Drops recursivelly all children
    // How: casts the children into boxes and then imaediatelly drops them
    fn drop(&mut self) {
        if let Some(left_ptr) = self.left {
            unsafe {
                let left = Box::from_raw(left_ptr.as_ptr());
                drop(left);
            }
        }
        if let Some(right_ptr) = self.right {
            unsafe {
                let right = Box::from_raw(right_ptr.as_ptr());
                drop(right);
            }
        }
    }
}

// Utilities to help maintain the sanity
impl<T, U: Ord> Node<T, U> {
    /// Changes the node parent \
    /// **DOES NOT** deallocate the old parent !!
    fn set_parent(&mut self, p: Option<NonNull<Node<T, U>>>) {
        unsafe {
            self.parent = p;
        }
    }

    /// Changes the node left \
    /// **DOES NOT** deallocate the old left !!    
    fn set_left(&mut self, l: Option<NonNull<Node<T, U>>>) {
        unsafe {
            self.left = l;
        }
    }

    /// Changes the node right \
    /// **DOES NOT** deallocate the old right !!
    fn set_right(&mut self, r: Option<NonNull<Node<T, U>>>) {
        unsafe {
            self.right = r;
        }
    }

    fn get_parent(&self) -> Option<NonNull<Node<T, U>>> {
        self.parent
    }

    fn get_left(&self) -> Option<NonNull<Node<T, U>>> {
        self.left
    }

    fn get_right(&self) -> Option<NonNull<Node<T, U>>> {
        self.right
    }

    fn get_height(node: Option<NonNull<Node<T, U>>>) -> usize {
        match node {
            Some(n) => unsafe { n.as_ref().height },
            None => 0,
        }
    }

    fn update_height(&mut self) {
        let l_height = Self::get_height(self.get_left());
        let r_height = Self::get_height(self.get_right());

        self.height = 1 + max(l_height, r_height);
    }

    fn get_size(node: Option<NonNull<Node<T, U>>>) -> usize {
        match node {
            Some(n) => unsafe { n.as_ref().size },
            None => 0,
        }
    }

    fn update_size(&mut self) {
        let l_size = Self::get_size(self.get_left());
        let r_size = Self::get_size(self.get_right());

        self.size = 1 + l_size + r_size;
    }

    fn update_balance(&mut self) {
        let l_height = Self::get_height(self.get_left());
        let r_height = Self::get_height(self.get_right());

        self.balance = l_height as isize - r_height as isize;
    }

    fn update_metadata(mut node: NonNull<Node<T, U>>) {
        unsafe {
            node.as_mut().update_height();
            node.as_mut().update_balance();
            node.as_mut().update_size();
        }
    }
}

impl<T, U: Ord> Node<T, U> {
    pub fn new(key: U, data: T) -> Self {
        Node {
            key,
            data,
            height: 0,
            size: 0,
            balance: 0,
            parent: None,
            right: None,
            left: None,
        }
    }
}

//  Rotations
impl<T, U: Ord> Node<T, U> {
    fn left_rotate(mut x: NonNull<Node<T, U>>) -> NonNull<Node<T, U>> {
        unsafe {
            let mut y = x.as_ref().get_right().unwrap_unchecked();
            let mut y_left = y.as_ref().get_left();
            let x_old_parent = x.as_ref().get_parent();

            x.as_mut().set_right(y_left);
            if let Some(mut node_y_left) = y_left {
                node_y_left.as_mut().set_parent(Some(x));
            }

            y.as_mut().set_left(Some(x));
            x.as_mut().set_parent(Some(y));

            y.as_mut().set_parent(x_old_parent);

            if let Some(mut parent) = x_old_parent {
                if Some(x) == parent.as_ref().get_left() {
                    parent.as_mut().set_left(Some(y));
                } else {
                    parent.as_mut().set_right(Some(y));
                }
            }

            Self::update_metadata(x);
            Self::update_metadata(y);

            return y;
        }
    }

    fn right_rotate(mut y: NonNull<Node<T, U>>) -> NonNull<Node<T, U>> {
        unsafe {
            let mut x = y.as_mut().get_left().unwrap_unchecked();
            let mut x_right = x.as_mut().get_right();
            let y_old_parent = y.as_ref().get_parent();

            y.as_mut().set_left(x_right);
            if let Some(mut node_x_right) = x_right {
                node_x_right.as_mut().set_parent(Some(y));
            }

            x.as_mut().set_right(Some(y));
            y.as_mut().set_parent(Some(x));

            x.as_mut().set_parent(y_old_parent);

            if let Some(mut parent) = y_old_parent {
                if Some(y) == parent.as_ref().get_left() {
                    parent.as_mut().set_left(Some(x));
                } else {
                    parent.as_mut().set_right(Some(x));
                }
            }

            Self::update_metadata(y);
            Self::update_metadata(x);

            return x;
        }
    }

    fn left_right_rotate(mut z: NonNull<Node<T, U>>) -> NonNull<Node<T, U>> {
        unsafe {
            let x = z.as_mut().get_left().unwrap_unchecked();
            let u = Self::left_rotate(x);
            let w = Self::right_rotate(z);
            return w;
        }
    }

    fn right_left_rotations(mut z: NonNull<Node<T, U>>) -> NonNull<Node<T, U>> {
        unsafe {
            let x = z.as_mut().get_right().unwrap_unchecked();
            let u = Self::right_rotate(x);
            let w = Self::right_rotate(z);
            return w;
        }
    }

    fn rebalance(mut node: NonNull<Self>) -> NonNull<Self> {
        unsafe {
            let balance = node.as_ref().balance;

            if balance > 1 {
                let left_ptr = node.as_ref().left.unwrap_unchecked();

                if left_ptr.as_ref().balance < 0 {
                    let new_left = Self::left_rotate(left_ptr);
                    node.as_mut().left = Some(new_left);
                }

                return Self::right_rotate(node);
            }

            if balance < -1 {
                let right_ptr = node.as_ref().right.unwrap_unchecked();

                if right_ptr.as_ref().balance > 0 {
                    let new_right = Self::right_rotate(right_ptr);
                    node.as_mut().right = Some(new_right);
                }

                return Self::left_rotate(node);
            }
        }

        node
    }
}

impl<T: Clone, U: Ord + Clone> Node<T, U> {
    pub fn insert(
        mut current: Option<NonNull<Node<T, U>>>,
        parent: Option<NonNull<Node<T, U>>>,
        key: U,
        data: T,
    ) -> NonNull<Node<T, U>> {
        unsafe {
            match current {
                None => {
                    let mut new_node = Node::new(key, data);
                    new_node.parent = parent;
                    let boxed = Box::new(new_node);
                    let node = NonNull::new_unchecked(Box::into_raw(boxed));
                    return node;
                }
                Some(mut c) => {
                    let key_clone = key.clone();
                    let curr_key = &c.as_ref().key;
                    let left = c.as_ref().get_left();
                    let right = c.as_ref().get_right();

                    match curr_key.cmp(&key) {
                        std::cmp::Ordering::Less => {
                            let new_left = Self::insert(right, Some(c), key, data);
                            c.as_mut().set_left(Some(new_left));
                        }
                        std::cmp::Ordering::Greater => {
                            let new_right = Self::insert(left, Some(c), key, data);
                            c.as_mut().set_right(Some(new_right));
                        }
                        std::cmp::Ordering::Equal => {
                            c.as_mut().data = data;
                        }
                    }

                    Node::update_metadata(c);

                    return Self::rebalance(c);
                }
            }
        }
    }

    pub fn delete(
        mut current: Option<NonNull<Node<T, U>>>,
        key: &U,
    ) -> Option<NonNull<Node<T, U>>> {
        unsafe {
            let mut c = match current {
                Some(ptr) => ptr,
                None => return None,
            };

            let curr_key = &c.as_ref().key;

            if key < curr_key {
                let left = c.as_ref().get_left();
                let new_left = Self::delete(left, key);
                c.as_mut().set_left(new_left);
            } else if key > curr_key {
                let right = c.as_ref().get_right();
                let new_right = Self::delete(right, key);
                c.as_mut().set_right(new_right);
            } else {
                let left = c.as_ref().get_left();
                let right = c.as_ref().get_right();
                let parent = c.as_ref().get_parent();

                match (left, right) {
                    (None, None) => {
                        let _ = Box::from_raw(c.as_ptr());
                        return None;
                    }
                    (Some(mut l), None) => {
                        l.as_mut().set_parent(parent);
                        c.as_mut().set_left(None);
                        let _ = Box::from_raw(c.as_ptr());
                        return Some(l);
                    }
                    (None, Some(mut r)) => {
                        r.as_mut().set_parent(parent);
                        c.as_mut().set_right(None);
                        let _ = Box::from_raw(c.as_ptr());
                        return Some(r);
                    }
                    (Some(_), Some(r)) => {
                        let mut successor = r;
                        while let Some(next_left) = successor.as_ref().get_left() {
                            successor = next_left;
                        }

                        let succ_key = successor.as_ref().key.clone();
                        c.as_mut().key = succ_key.clone();
                        c.as_mut().data = successor.as_ref().data.clone();

                        let new_right = Self::delete(Some(r), &succ_key);
                        c.as_mut().set_right(new_right);
                    }
                }
            }

            Self::update_metadata(c);

            let rebalanced = Self::rebalance(c);

            return Some(rebalanced);
        }
    }

    pub fn kth(current: Option<NonNull<Node<T, U>>>, mut k: usize) -> Option<NonNull<Node<T, U>>> {
        let mut curr = current;

        while let Some(node_ptr) = curr {
            unsafe {
                let right_size = Self::get_size(node_ptr.as_ref().get_right());

                if right_size >= k {
                    curr = node_ptr.as_ref().get_right();
                } else if right_size == k - 1 {
                    return Some(node_ptr);
                } else {
                    k = k - (right_size + 1);

                    curr = node_ptr.as_ref().get_left();
                }
            }
        }
        None
    }

    pub fn rank(current: Option<NonNull<Node<T, U>>>, x: &U) -> usize {
        let mut curr = current;
        let mut count = 0;

        while let Some(node_ptr) = curr {
            unsafe {
                let curr_key = &node_ptr.as_ref().key;

                if curr_key > x {
                    count += 1;
                    count += Self::get_size(node_ptr.as_ref().get_right());

                    curr = node_ptr.as_ref().get_left();
                } else {
                    curr = node_ptr.as_ref().get_right();
                }
            }
        }
        count
    }
}
