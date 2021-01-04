#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
/*
use std::cmp::max;
use std::mem;
use std::{thread, time};
use util::print_indent;
use std::cell::RefCell;
*/

// use std::fmt::*;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;

use ::util::*;

pub fn main() {
    // try_build_from_string();
    try_get_branch();
    // try_show_branches();
    // try_build_from_vector();
    // run_internal_tests();
    // try_interior_mutability();
    // try_transform();
    // try_mirror();
}

const ROOT_INDEX: usize = 0;
const NONE_CHAR: char = '.';
const TREE_DEF_STRING_1: &str = "abfcegh.d....ik............j";

pub trait ChainDebug<T: Debug> {
    fn chain_debug(self, label: &str) -> T;
}

impl ChainDebug<Option<usize>> for Option<usize> {
    fn chain_debug(self, label: &str) -> Option<usize> {
        println!("{}: {:?}", &label, &self);
        self
    }
}

enum NodeChildType {
    Root,
    LeftChild,
    RightChild,
}

pub struct Tree<T> {
    vec: Vec<Option<T>>,
    debug_function: Option<fn(&T) -> String>,
}

pub struct Branch<'a, T> {
    root: &'a Tree<T>,
    index: usize,
}

pub struct BranchMut<'a, T> {
    root: &'a mut Tree<T>,
    index: usize,
}

impl<T> Tree<T> {
    pub fn from_vector(vec: Vec<Option<T>>, debug_function: Option<fn(&T) -> String>) -> Self {
        Self {
            vec,
            debug_function,
        }
    }

    fn last_item_index(&self) -> Option<usize> {
        Self::vec_last_item_index(&self.vec)
    }

    fn vec_last_item_index(vec: &Vec<Option<T>>) -> Option<usize> {
        vec.iter().rposition(|x| x.is_some())
    }

    pub fn index_to_parent_index(&self, index: usize) -> Option<usize> {
        assert!(index >= ROOT_INDEX);
        if index == ROOT_INDEX {
            return None;
        }
        Some(((index + 1) / 2) - 1)
    }

    fn get_branch(&self, index: usize) -> Option<Branch<T>> {
        self.vec.get(index).map(|x| Branch { root: self, index })
    }

    fn as_branch(&self) -> Branch<T> {
        self.get_branch(ROOT_INDEX).unwrap()
    }

    fn get(&self, index: usize) -> Option<&T> {
        // self.vec.get() returns on Option<&Option<T>>. We want to return None if the index is
        // beyond the end of the vector or if the item is in the vector but the value there is
        // None. We don't care about the distinction so flatten it.
        // self.vec.get(index).flatten()
        self.vec.get(index)?.as_ref()
    }

    #[inline]
    fn parent(&self) -> Option<Branch<T>> {
        None
    }

    fn child_indexes(&self, index: usize) -> Vec<usize> {
        let mut vec = vec![];
        self.left_child_index(index).map(|x| vec.push(x));
        self.right_child_index(index).map(|x| vec.push(x));
        vec
    }

    #[inline]
    fn left_child_index(&self, index: usize) -> Option<usize> {
        assert!(index >= ROOT_INDEX);
        let i = ((index + 1) * 2) - 1;
        self.vec.get(i)?.as_ref().and(Some(i))
    }

    #[inline]
    fn right_child_index(&self, index: usize) -> Option<usize> {
        assert!(index >= ROOT_INDEX);
        let i = (index + 1) * 2;
        self.vec.get(i)?.as_ref().and(Some(i))
    }

    fn index_to_debug_string(&self, index: usize) -> String {
        match self.vec.get(index) {
            Some(item) => {
                // item.as_ref().map_or("None".to_string(), |x| format!("{:?}", x))
                match self.debug_function {
                    Some(func) => item.as_ref().map_or("None".to_string(), func),
                    None => "{no debug function defined}".to_string(),
                }
            }
            None => "{not in vector}".to_string(),
        }
    }

    fn describe_deep_from_index(
        &self,
        s: &mut String,
        index: usize,
        depth: usize,
        max_depth: Option<usize>,
    ) {
        // let this_line = self.get(index).map_or("None".to_string(), |x| format!("{:?}", x));
        let left_right = match self.node_child_type(index) {
            NodeChildType::Root => "root:  ".to_string(),
            NodeChildType::LeftChild => "left:  ".to_string(),
            _ => "right: ".to_string(),
        };
        let index_places = self.vec.len().to_string().len();
        let index_string = format!("{:>1$} ", index, index_places);
        let this_line = self.index_to_debug_string(index);
        let this_line = format!(
            "{}{}{}{}\n",
            index_string,
            ". ".repeat(depth),
            left_right,
            this_line
        );
        s.push_str(&this_line);
        if max_depth.map_or(true, |max| depth < max) {
            for child_index in self.child_indexes(index) {
                self.describe_deep_from_index(s, child_index, depth + 1, max_depth);
            }
        }
    }

    fn show(&self, index: usize) {
        let mut s = String::new();
        self.describe_deep_from_index(&mut s, ROOT_INDEX, 1, None);
        println!("{}", s);
    }

    /*
    fn print_deep_from_index(&self, index: usize, depth: usize, max_depth: Option<usize>) {
        let mut s = String::new();
        self.describe_deep_from_index(&mut s, index, depth, max_depth.map(|x| x + depth));
        println!(s);
    }
    */

    fn node_child_type(&self, index: usize) -> NodeChildType {
        if index == ROOT_INDEX {
            return NodeChildType::Root;
        } else if index % 2 == 1 {
            return NodeChildType::LeftChild;
        } else {
            return NodeChildType::RightChild;
        }
    }
}

impl<T: Display> Tree<T> {
    pub fn from_vector_display(
        vec: Vec<Option<T>>,
        debug_function: Option<fn(&T) -> String>,
    ) -> Self {
        Self {
            vec,
            debug_function: Some(|x| ToString::to_string(x)),
        }
    }
}

impl<T> Branch<'_, T> {
    #[inline]
    fn parent(&self) -> Option<Branch<T>> {
        let parent_index = self.root.index_to_parent_index(self.index);
        self.root.get_branch(parent_index.unwrap())
    }
}

impl<T> Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Multiline.
            // let branch = self.as_branch();
            // write!(f, "BinaryTreeRoot\n{:#?}", &branch)
            let mut s = String::new();
            self.describe_deep_from_index(&mut s, ROOT_INDEX, 1, None);
            write!(f, "\n{}\n{}", "Root", s)
        } else {
            // Single line.
            write!(f, "Root")
        }
    }
}

impl<T> Debug for Branch<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            // Multiline.
            // write!(f, "BinaryTreeBranch: index = {}\ndummy line", self.index)
            let mut s = String::new();
            self.root
                .describe_deep_from_index(&mut s, self.index, 1, None);
            write!(f, "\n{}\n{}", "Branch", s)
        } else {
            // Single line.
            write!(f, "Branch: index = {}", self.index)
        }
    }
}

/*
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut s = String::new();
            self.describe_deep(&mut s, 0);
            write!(f, "{}", s)
        } else {
            let s = self.describe_one_line();
            write!(f, "{}", s)
        }
    }

*/

/*

#[inline]
fn right_sibling_index(&self, index: usize) -> Option<usize> {
    if index == ROOT_INDEX || !self.is_left_child(index) {
        return None;
    }
    self.right_child_index(self.parent_index(index).unwrap())
}

#[inline]
fn is_left_child(&self, index: usize) -> bool {
    index > ROOT_INDEX && index % 2 == 1
}

#[inline]
fn next_index_depth(&self, index: usize) -> Option<usize> {
    self.next_index_depth_internal(index, false)
}

#[inline]
fn next_index_depth_internal(&self, index: usize, skip_child_nodes: bool) -> Option<usize> {
    assert!(index >= ROOT_INDEX);
    let parent_index = self.parent_index(index);
    dbg!(&index);

    /*
    let left_index = self.left_index(index);
    dbg!(&left_index);
    if left_index.is_some() {
        return left_index;
    }

    let right_index = self.right_index(index);
    dbg!(&right_index);
    if right_index.is_some() {
        return right_index;
    }

    dbg!(&parent_other_child_index);
    if parent_other_child_index.is_some() {
        return parent_other_child_index;
    }

    if let Some(parent_i) = parent_index {
        dbg!(&parent_i);
        return self.next_index_depth(parent_i);
    }

    None
    */
    match skip_child_nodes {
        true => None,
        false => {
            self.left_child_index(index).chain_debug("left_child_index")
            .or(self.right_child_index(index)).chain_debug("right_child_index")
        },
    }
        .or(self.right_sibling_index(index)).chain_debug("right_sibling_index")
        .or(parent_index.and_then(|parent_i| self.next_index_depth_internal(parent_i, true)))
}

pub fn as_indexes_depth(&self) -> Vec<usize> {
    self.as_indexes_depth_internal(ROOT_INDEX)
}

fn as_indexes_depth_internal(&self, index: usize) -> Vec<usize> {
    assert!(index >= ROOT_INDEX);
    let mut vec = vec!();
    let mut index_opt = Some(index);
    while index_opt.is_some() {
        let one_index = index_opt.unwrap();
        vec.push(one_index);
        index_opt = self.next_index_depth(one_index);
    }
    vec
}

pub fn as_elements_depth(&self) -> Vec<&T> {
    self.as_elements_depth_internal(ROOT_INDEX)
}

fn as_elements_depth_internal(&self, index: usize) -> Vec<&T> {
    assert!(index >= ROOT_INDEX);
    self.as_indexes_depth_internal(index).iter().map(|i| self.vec[*i].as_ref().unwrap()).collect::<Vec<&T>>()
}
*/
/*
pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter { next: ROOT_INDEX }
}
*/

/*
fn describe_deep(&self, s: &mut String, depth: usize) {
    s.push_str(&format!(
        "{}\n",
        format_indent(depth, &(self.describe_one_line()))
    ));
    if depth < DEBUG_TRIE_MAX_DEPTH {
        for child_node in self
            .children
            .values()
            .map(|x| x.borrow())
            .take(DEBUG_TRIE_MAX_CHILDREN)
            {
                child_node.describe_deep(s, depth + 1);
            }
    }
}
*/

impl Tree<char> {
    pub fn from_string(s: &str, none_char: char) -> Self {
        let debug_function: fn(&char) -> String = |x| x.to_string();
        Self::from_vector(string_to_char_vector(&s, none_char), Some(debug_function))
    }

    /*
    pub fn test() {
        // Empty tree.
        let t1 = Self::from_string("", NONE_CHAR);
        assert_eq!(None, t1.last_item_index());
        let index = ROOT_INDEX;
        assert_eq!(None, t1.parent_index(index));
        assert_eq!(None, t1.left_child_index(index));
        assert_eq!(None, t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(None, t1.right_sibling_index(index));
        assert_eq!(false, t1.is_left_child(index));
        assert_eq!(None, t1.next_index_depth(index));

        let t1 = Self::from_string("a", NONE_CHAR);
        assert_eq!(Some(ROOT_INDEX), t1.last_item_index());
        let index = ROOT_INDEX;
        assert_eq!(None, t1.parent_index(index));
        assert_eq!(None, t1.left_child_index(index));
        assert_eq!(None, t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(None, t1.right_sibling_index(index));
        assert_eq!(false, t1.is_left_child(index));
        assert_eq!(None, t1.next_index_depth(index));

        let t1 = Self::from_string("ab", NONE_CHAR);
        assert_eq!(Some(1), t1.last_item_index());
        let index = ROOT_INDEX;
        assert_eq!(None, t1.parent_index(index));
        assert_eq!(Some(1), t1.left_child_index(index));
        assert_eq!(None, t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(None, t1.right_sibling_index(index));
        assert_eq!(false, t1.is_left_child(index));
        assert_eq!(Some(1), t1.next_index_depth(index));
        let index = 1;
        assert_eq!(Some(ROOT_INDEX), t1.parent_index(index));
        assert_eq!(None, t1.left_child_index(index));
        assert_eq!(None, t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(None, t1.right_sibling_index(index));
        assert_eq!(true, t1.is_left_child(index));
        assert_eq!(None, t1.next_index_depth(index));

        let t1 = Self::from_string("a.b", NONE_CHAR);
        assert_eq!(Some(2), t1.last_item_index());
        let index = ROOT_INDEX;
        assert_eq!(None, t1.parent_index(index));
        assert_eq!(None, t1.left_child_index(index));
        assert_eq!(Some(2), t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(None, t1.right_sibling_index(index));
        assert_eq!(false, t1.is_left_child(index));
        assert_eq!(Some(2), t1.next_index_depth(index));
        let index = 1;
        assert_eq!(Some(ROOT_INDEX), t1.parent_index(index));
        assert_eq!(None, t1.left_child_index(index));
        assert_eq!(None, t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(Some(2), t1.right_sibling_index(index));
        assert_eq!(true, t1.is_left_child(index));
        assert_eq!(None, t1.next_index_depth(index));
        let index = 2;
        assert_eq!(Some(ROOT_INDEX), t1.parent_index(index));
        assert_eq!(None, t1.left_child_index(index));
        assert_eq!(None, t1.right_child_index(index));
        // assert_eq!(None, t1.left_sibling_index(index));
        assert_eq!(None, t1.right_sibling_index(index));
        assert_eq!(false, t1.is_left_child(index));
        assert_eq!(None, t1.next_index_depth(index));
    }
    */
}

fn make_tree_1() -> Tree<char> {
    Tree::from_string(TREE_DEF_STRING_1, NONE_CHAR)
}

fn try_build_from_string() {
    let r1 = make_tree_1();
    // dbg!(t1.as_indexes_depth());
    dbg!(&r1);
    dbg!(&r1.parent());
}

fn try_get_branch() {
    let r1 = make_tree_1();
    let b1 = r1.get_branch(1).unwrap();
    dbg!(&r1);
    dbg!(&r1.parent());
    dbg!(&b1);
    dbg!(&b1.parent());
}

/*

fn run_internal_tests() {
    BinaryTree::<char>::test();
}



/*
impl <T: Display> BinaryTree<T> {
    fn describe_deep(&self, s: &mut String, depth: usize) {
        s.push_str(&format!(
            "{}\n",
            format_indent(depth, &(self.describe_one_line()))
        ));
        if depth < DEBUG_TRIE_MAX_DEPTH {
            for child_node in self
                .children
                .values()
                .map(|x| x.borrow())
                .take(DEBUG_TRIE_MAX_CHILDREN)
                {
                    child_node.describe_deep(s, depth + 1);
                }
        }
    }
}

impl <T: Display> BinaryTree<T> {
    fn describe_deep(&self, s: &mut String, depth: usize) {
        s.push_str(&format!(
            "{}\n",
            format_indent(depth, &(self.describe_one_line()))
        ));
        if depth < DEBUG_TRIE_MAX_DEPTH {
            for child_node in self
                .children
                .values()
                .map(|x| x.borrow())
                .take(DEBUG_TRIE_MAX_CHILDREN)
                {
                    child_node.describe_deep(s, depth + 1);
                }
        }
    }
}
*/
/*
impl <T: Display> std::fmt::Debug for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {


        if f.alternate() {
            let mut s = String::new();
            self.describe_deep(&mut s, ROOT_INDEX);
            write!(f, "{}", s)
        } else {
            let s = self.describe_one_line();
            write!(f, "{}", s)
        }
    }
}

impl <T: Display> std::fmt::Debug for BinaryTreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {


        if f.alternate() {
            let mut s = String::new();
            self.describe_deep(&mut s, ROOT_INDEX);
            write!(f, "{}", s)
        } else {
            let s = self.describe_one_line();
            write!(f, "{}", s)
        }
    }
}

*/

/*

    fn from_vector_internal(vec: &mut Vec<Option<T>>, index: usize) -> Option<Self> {
        vec
            .get_mut(index)
            .and_then(|x| x.take())
            .map(|x| {
                BinaryTreeNode {
                    val: x,
                    left: Self::from_vector_internal(vec, Self::index_left(index)).map(|x| Box::new(x)),
                    right: Self::from_vector_internal(vec, Self::index_right(index)).map(|x| Box::new(x)),
                    calc_expensive_value: RefCell::new(None),
                }
            })
    }

    fn index_left(index: usize) -> usize {
        ((index + 1) * 2) - 1
    }

    fn index_right(index: usize) -> usize {
        (index + 1) * 2
    }

    pub fn size(&self) -> usize {
        1
            + self.left.as_ref().map(|x| x.size()).unwrap_or(0)
            + self.right.as_ref().map(|x| x.size()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        1 + max(
            self.left.as_ref().map(|x| x.height()).unwrap_or(0),
            self.right.as_ref().map(|x| x.height()).unwrap_or(0))
    }

    pub fn calc_expensive(&self, delay_msec: u64) -> usize {
        // This was inspired by the example on https://doc.rust-lang.org/std/cell/index.html.
        *self.calc_expensive_value.borrow_mut()
            .get_or_insert_with(|| {
                thread::sleep(time::Duration::from_millis(delay_msec));
                10000
                    + self.left.as_ref().map(|x| x.calc_expensive(delay_msec)).unwrap_or(17)
                    + self.right.as_ref().map(|x| x.calc_expensive(delay_msec)).unwrap_or(23)
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

    pub fn transform<U>(&self, func: &dyn Fn(&T) -> U) -> BinaryTreeNode<U> {
        BinaryTreeNode::from_vector(self.as_vector(func))
    }

    pub fn as_vector<U>(&self, func: &dyn Fn(&T) -> U) -> Vec<Option<U>> {
        let max_nodes = 2u8.pow(self.height() as u32) as usize - 1;
        let mut vec = Vec::with_capacity(max_nodes);
        for i in 0..max_nodes {
            vec.push(None);
        }
        self.as_vector_internal(func, &mut vec, ROOT_INDEX);
        vec
    }

    pub fn as_vector_internal<U>(&self, func: &dyn Fn(&T) -> U, vec: &mut Vec<Option<U>>, index: usize) {
        let (val, left, right) = (func(&self.val), &self.left, &self.right);
        vec[index] = Some(val);
        left.as_ref().map(|x| x.as_vector_internal(func, vec, Self::index_left(index)));
        right.as_ref().map(|x| x.as_vector_internal(func, vec, Self::index_right(index)));
    }

    pub fn mirror(&mut self) {
        mem::swap(&mut self.left, &mut self.right);
        self.left.as_mut().map(|x| x.mirror());
        self.right.as_mut().map(|x| x.mirror());
    }
}

impl<T: Display> BinaryTreeNode<T> {
    pub fn print(&self) {
        self.print_internal(ROOT_INDEX, "root:");
    }

    fn print_internal(&self, depth: usize, label: &str) {
        print_indent(depth, format!("{:<6} {}", label, &self.val).as_ref());
        self.left.as_ref().map(|x| x.print_internal(depth + 1, "left:"));
        self.right.as_ref().map(|x| x.print_internal(depth + 1, "right:"));
    }

    pub fn as_vector_display(&self) -> Vec<Option<String>> {
        self.as_vector(&|x| format!("{}", x))
    }
}

impl<T: Clone> BinaryTreeNode<T> {
    pub fn clone(&self, mirror: bool) {
        unimplemented!()
    }
}

impl BinaryTreeNode<char> {
    pub fn from_string(s: &str, none_char: char) -> Self {
        Self::from_vector(string_to_char_vector(&s, none_char))
    }

    pub fn as_string(&self, none_char: char) -> String {
        unimplemented!()
    }
}

fn make_tree_1() -> BinaryTreeNode<char> {
    BinaryTreeNode::from_string(TREE_DEF_STRING_1, NONE_CHAR)
}

fn try_build_from_vector() {
    let v1 = string_to_char_vector(&TREE_DEF_STRING_1, NONE_CHAR);
    let s1 = char_vector_to_string(&v1, NONE_CHAR);
    assert_eq!(TREE_DEF_STRING_1, s1);
    // let v1_display = vec_copy_for_display(&v1);
    // dbg!(&v.len(), &v);
    let t1 = BinaryTreeNode::from_vector(v1);
    t1.print();
    assert_eq!(11, t1.size());
    assert_eq!(5, t1.height());

    let v2 = t1.into_vector(true);
    let s2 = char_vector_to_string(&v2, NONE_CHAR);
    assert_eq!(TREE_DEF_STRING_1, s2);

    let t2 = BinaryTreeNode::from_vector(string_to_char_vector(&s2, NONE_CHAR));
    t2.print();
}

fn try_build_from_string() {
    let t1 = BinaryTreeNode::from_string(TREE_DEF_STRING_1, NONE_CHAR);
    t1.print();
    assert_eq!(11, t1.size());
    assert_eq!(5, t1.height());

    let v2 = t1.into_vector(true);
    let s2 = char_vector_to_string(&v2, NONE_CHAR);
    assert_eq!(TREE_DEF_STRING_1, s2);

    let t2 = BinaryTreeNode::from_vector(string_to_char_vector(&s2, NONE_CHAR));
    t2.print();

    // Create a non-truncated string that might have periods at the end.
    let s3 = char_vector_to_string(&t2.into_vector(false), NONE_CHAR);
    dbg!(s3);
}

fn try_interior_mutability() {
    let t1 = make_tree_1();
    let delay: u64 = 100;
    util::print_elapsed(true, "first run", "", || {
        dbg!(t1.calc_expensive(delay));
    });
    util::print_elapsed(true, "second run", "", || {
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
*/

/*
#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}
*/

/*
fn chain_display<T: Display>(val: T, label: &str) -> T {
    println!("{}", &val);
    val
}
*/

*/

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
