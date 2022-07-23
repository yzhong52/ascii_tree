use std::rc::Rc;

use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode {
    pub label: String,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}
