#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::{Debug, Display};

const ROOT_INDEX: usize = 0;
const NONE_CHAR: char = '.';
const TREE_DEF_STRING_1: &str = "abfcegh.d....ik............j";

fn main() {}

enum BinaryTree<'a, T> {
    Root {
        vec: Vec<Option<T>>,
    },
    Branch {
        tree: &'a BinaryTree<'a, T>,
        index: usize,
    },
}

impl<'a, T> BinaryTree<'a, T> {
    pub fn from_vector(vec: Vec<Option<T>>) -> Self {
        BinaryTree::Root { vec }
    }

    /*
    fn last_item_index(&self) -> Option<usize> {
        match self {
            BinaryTree::Root { .. } => {
                Self::vec_last_item_index(&self.vec)
            }
            _ => None
        }
    }
    */

    fn vec_last_item_index(vec: &Vec<Option<T>>) -> Option<usize> {
        vec.iter().rposition(|x| x.is_some())
    }

    /*
    #[inline]
    fn parent_index(&self, index: usize) -> Option<usize> {
        assert!(index >= ROOT_INDEX);
        if index == ROOT_INDEX {
            return None;
        }
        Some(((index + 1)/ 2) - 1)
    }
    */
    /*
    #[inline]
    fn child_indexes(&self, index: usize) -> (Option<usize>, Option<usize>) {
        assert!(index >= ROOT_INDEX);
        let i = index * 2;
        let left = self.vec.get(i)?.and(Some(i));
        let i = i + 1;
        let right = self.vec.get(i)?.and(Some(i));
        (left, right)
    }
    */

    /*
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
}
