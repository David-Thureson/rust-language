#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Display;
use std::mem;
use std::{thread, time};
use util::format::*;
use util::date_time::*;

// This uses the binary tree structure used in several LeetCode problems, except that it's
// generic instead of holding i32 values so that there is no Copy or Clone and thus a better
// challenges. Also instead of Rc<RefCell<TreeNode>> we have simply Box<TreeNode>.

// The test is to:
//  1. Create a tree from a vector of Option<T>.
//  2. Iterate depth-first.
//  3. Turn a tree into a vector of Option<T>.

// One LeetCode example is 101. Symmetric Tree: https://leetcode.com/problems/symmetric-tree/.
// See the leet_code project for more.

/* LeetCode's tree is:
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
*/

pub fn main() {
    // try_build_from_vector();
    // try_build_from_string();
    try_interior_mutability();
    // try_transform();
    // try_mirror();
}

const NONE_CHAR: char = '.';
const TREE_DEF_STRING_1: &str = "abfcegh.d....ik............j";

pub struct NestedBinaryTreeNode<T> {
    pub val: T,
    pub left: Option<Box<NestedBinaryTreeNode<T>>>,
    pub right: Option<Box<NestedBinaryTreeNode<T>>>,
    calc_expensive_value: RefCell<Option<usize>>,
}

impl<T> NestedBinaryTreeNode<T> {
    /*
    fn new(val: T) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
    */

    pub fn from_vector(mut vec: Vec<Option<T>>) -> Self {
        assert!(vec.len() > 0);
        Self::from_vector_internal(&mut vec, 0).unwrap()
    }

    fn from_vector_internal(vec: &mut Vec<Option<T>>, index: usize) -> Option<Self> {
        vec.get_mut(index)
            .and_then(|x| x.take())
            .map(|x| NestedBinaryTreeNode {
                val: x,
                left: Self::from_vector_internal(vec, Self::index_left(index)).map(|x| Box::new(x)),
                right: Self::from_vector_internal(vec, Self::index_right(index))
                    .map(|x| Box::new(x)),
                calc_expensive_value: RefCell::new(None),
            })
    }

    fn index_left(index: usize) -> usize {
        ((index + 1) * 2) - 1
    }

    fn index_right(index: usize) -> usize {
        (index + 1) * 2
    }

    pub fn size(&self) -> usize {
        1 + self.left.as_ref().map(|x| x.size()).unwrap_or(0)
            + self.right.as_ref().map(|x| x.size()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        1 + max(
            self.left.as_ref().map(|x| x.height()).unwrap_or(0),
            self.right.as_ref().map(|x| x.height()).unwrap_or(0),
        )
    }

    pub fn calc_expensive(&self, delay_msec: u64) -> usize {
        // This was inspired by the example on https://doc.rust-lang.org/std/cell/index.html.
        *self
            .calc_expensive_value
            .borrow_mut()
            .get_or_insert_with(|| {
                thread::sleep(time::Duration::from_millis(delay_msec));
                10000
                    + self
                        .left
                        .as_ref()
                        .map(|x| x.calc_expensive(delay_msec))
                        .unwrap_or(17)
                    + self
                        .right
                        .as_ref()
                        .map(|x| x.calc_expensive(delay_msec))
                        .unwrap_or(23)
            })
    }

    pub fn into_vector(self, truncate: bool) -> Vec<Option<T>> {
        let max_nodes = 2u8.pow(self.height() as u32) as usize - 1;
        // let mut vec= vec![None; max_nodes];
        let mut vec = Vec::with_capacity(max_nodes);
        for i in 0..max_nodes {
            vec.push(None);
        }
        self.into_vector_internal(&mut vec, 0);
        if truncate {
            let last_real_item_index = vec.iter().rposition(|x| x.is_some()).unwrap();
            vec.truncate(last_real_item_index + 1);
        }
        vec
    }

    pub fn into_vector_internal(self, vec: &mut Vec<Option<T>>, index: usize) {
        let (val, left, right) = (self.val, self.left, self.right);
        vec[index] = Some(val);
        left.map(|x| x.into_vector_internal(vec, Self::index_left(index)));
        right.map(|x| x.into_vector_internal(vec, Self::index_right(index)));
    }

    pub fn transform<U>(&self, func: &dyn Fn(&T) -> U) -> NestedBinaryTreeNode<U> {
        NestedBinaryTreeNode::from_vector(self.as_vector(func))
    }

    pub fn as_vector<U>(&self, func: &dyn Fn(&T) -> U) -> Vec<Option<U>> {
        let max_nodes = 2u8.pow(self.height() as u32) as usize - 1;
        let mut vec = Vec::with_capacity(max_nodes);
        for i in 0..max_nodes {
            vec.push(None);
        }
        self.as_vector_internal(func, &mut vec, 0);
        vec
    }

    pub fn as_vector_internal<U>(
        &self,
        func: &dyn Fn(&T) -> U,
        vec: &mut Vec<Option<U>>,
        index: usize,
    ) {
        let (val, left, right) = (func(&self.val), &self.left, &self.right);
        vec[index] = Some(val);
        left.as_ref()
            .map(|x| x.as_vector_internal(func, vec, Self::index_left(index)));
        right
            .as_ref()
            .map(|x| x.as_vector_internal(func, vec, Self::index_right(index)));
    }

    pub fn mirror(&mut self) {
        mem::swap(&mut self.left, &mut self.right);
        self.left.as_mut().map(|x| x.mirror());
        self.right.as_mut().map(|x| x.mirror());
    }
}

impl<T: Display> NestedBinaryTreeNode<T> {
    pub fn print(&self) {
        self.print_internal(0, "root:");
    }

    fn print_internal(&self, depth: usize, label: &str) {
        println_indent_space(depth, format!("{:<6} {}", label, &self.val).as_ref());
        self.left
            .as_ref()
            .map(|x| x.print_internal(depth + 1, "left:"));
        self.right
            .as_ref()
            .map(|x| x.print_internal(depth + 1, "right:"));
    }

    pub fn as_vector_display(&self) -> Vec<Option<String>> {
        self.as_vector(&|x| format!("{}", x))
    }
}

impl<T: Clone> NestedBinaryTreeNode<T> {
    pub fn clone(&self, mirror: bool) {
        unimplemented!()
    }
}

impl NestedBinaryTreeNode<char> {
    pub fn from_string(s: &str, none_char: char) -> Self {
        Self::from_vector(string_to_char_vector(&s, none_char))
    }

    pub fn as_string(&self, none_char: char) -> String {
        unimplemented!()
    }
}

fn make_tree_1() -> NestedBinaryTreeNode<char> {
    NestedBinaryTreeNode::from_string(TREE_DEF_STRING_1, NONE_CHAR)
}

fn try_build_from_vector() {
    let v1 = string_to_char_vector(&TREE_DEF_STRING_1, NONE_CHAR);
    let s1 = char_vector_to_string(&v1, NONE_CHAR);
    assert_eq!(TREE_DEF_STRING_1, s1);
    // let v1_display = vec_copy_for_display(&v1);
    // dbg!(&v.len(), &v);
    let t1 = NestedBinaryTreeNode::from_vector(v1);
    t1.print();
    assert_eq!(11, t1.size());
    assert_eq!(5, t1.height());

    let v2 = t1.into_vector(true);
    let s2 = char_vector_to_string(&v2, NONE_CHAR);
    assert_eq!(TREE_DEF_STRING_1, s2);

    let t2 = NestedBinaryTreeNode::from_vector(string_to_char_vector(&s2, NONE_CHAR));
    t2.print();
}

fn try_build_from_string() {
    let t1 = NestedBinaryTreeNode::from_string(TREE_DEF_STRING_1, NONE_CHAR);
    t1.print();
    assert_eq!(11, t1.size());
    assert_eq!(5, t1.height());

    let v2 = t1.into_vector(true);
    let s2 = char_vector_to_string(&v2, NONE_CHAR);
    assert_eq!(TREE_DEF_STRING_1, s2);

    let t2 = NestedBinaryTreeNode::from_vector(string_to_char_vector(&s2, NONE_CHAR));
    t2.print();

    // Create a non-truncated string that might have periods at the end.
    let s3 = char_vector_to_string(&t2.into_vector(false), NONE_CHAR);
    dbg!(s3);
}

fn try_interior_mutability() {
    let t1 = make_tree_1();
    let delay: u64 = 100;
       print_elapsed(true, "first run", "", || {
        dbg!(t1.calc_expensive(delay));
    });
    print_elapsed(true, "second run", "", || {
        dbg!(t1.calc_expensive(delay));
    });
}

fn try_transform() {
    let t1 = make_tree_1();
    let t2 = t1.transform(&|x| x.to_uppercase());
    t1.print();
    t2.print();
}

fn try_mirror() {
    let mut t1 = make_tree_1();
    t1.print();
    t1.mirror();
    t1.print();
}

fn string_to_char_vector(s: &str, none_char: char) -> Vec<Option<char>> {
    s.chars()
        .map(|x| {
            // Use a straight == comparison instead of match since the period char was matching to
            // every char.
            if x == none_char {
                None
            } else {
                Some(x)
            }
            /*
            match x {
                '.' => None,
                _ => Some(x),
            }
            */
        })
        .collect()
}

fn char_vector_to_string(vec: &Vec<Option<char>>, none_char: char) -> String {
    vec.iter().map(|x| x.unwrap_or(none_char)).collect()
}
