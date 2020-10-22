// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::helper(root, &mut res);
        res
    }
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        match root {
            None => {},
            Some(node_ptr) => {
                Self::helper(node_ptr.borrow().left.clone(),v);
                Self::helper(node_ptr.borrow().right.clone(),v);
                v.push(node_ptr.borrow().val);
            }
        }
    }
}