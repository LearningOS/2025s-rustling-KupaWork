/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        //step1 comfirm if root is None or not
        //step2 if root is None, insert value
        //step3 if root is not None, compare value with root value
        //step4 if value is less than root value, insert into left subtree
        //step5 if value is greater than root value, insert into right subtree
        //step6 if value is equal to root value, do nothing
        //step7 again, if left or right subtree is None, insert value
        //step8 if left or right subtree is not None, call insert function recursively
        match self.root {
            Some(ref mut node) => { //if matched，get the ref mut of node
                match node.value.cmp(&value) {
                    Ordering::Less => {
                        if node.right.is_none() {
                            node.right = Some(Box::new(TreeNode::new(value)));
                        } else {
                            node.right.as_mut().unwrap().insert(value);
                        }
                    }
                    Ordering::Greater => {
                        if node.left.is_none() {
                            node.left = Some(Box::new(TreeNode::new(value)));
                        } else {
                            node.left.as_mut().unwrap().insert(value);
                        }
                    }
                    Ordering::Equal => {} // Do nothing for duplicates
                }
            }
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        //step1 comfirm if root is None or not
        //step2 if root is None, return false
        //step3 if root is not None, compare value with root value
        //step4 if value is less than root value, search in left subtree
        //step5 if value is greater than root value, search in right subtree
        //step6 if value is equal to root value, return true
        //step7 again, if left or right subtree is None, return false
        //step8 if left or right subtree is not None, call search function recursively
        match &self.root {
            Some(node) => {
                match node.value.cmp(&value) {
                    Ordering::Less => {
                        if let Some(ref right) = node.right {
                            right.search(value)
                        } else {
                            false
                        }
                    }
                    Ordering::Greater => {
                        if let Some(ref left) = node.left {
                            left.search(value)
                        } else {
                            false
                        }
                    }
                    Ordering::Equal => true,
                }
            }
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        //step1 comfirm if value is less than root value
        //step2 if value is less than root value, insert into left subtree
        //step3 if value is greater than root value, insert into right subtree
        //step4 if value is equal to root value, do nothing
        //step5 again, if left or right subtree is None, insert value
        //step6 if left or right subtree is not None, call insert function recursively
        match self.value.cmp(&value) {
            Ordering::Less => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {} // Do nothing for duplicates
        }
    }

    fn search(&self, value: T) -> bool {
        //TODO
        //step1 comfirm if value is less than root value
        //step2 if value is less than root value, search in left subtree
        //step3 if value is greater than root value, search in right subtree
        //step4 if value is equal to root value, return true
        //step5 again, if left or right subtree is None, return false
        //step6 if left or right subtree is not None, call search function recursively
        match self.value.cmp(&value) {
            Ordering::Greater => {
                if let Some(ref left) = self.left {
                    left.search(value)
                } else {
                    false
                }
            }
            Ordering::Less => {
                if let Some(ref right) = self.right {
                    right.search(value)
                } else {
                    false
                }
            }
            Ordering::Equal => true,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
