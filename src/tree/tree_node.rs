use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub label: String,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn from_label(label: &str) -> Self {
        TreeNode {
            label: label.to_string(),
            children: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn new(label: &str, children: Vec<TreeNode>) -> Self {
        TreeNode {
            label: label.to_string(),
            children: children
                .into_iter()
                .map(|child| Rc::new(RefCell::new(child)))
                .collect(),
        }
    }
}
