#[derive(Debug)]
pub struct TreeNode {
    pub label: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn from_label(label: &str) -> Self {
        TreeNode {
            label: label.to_string(),
            children: vec![],
        }
    }

    #[cfg(test)]
    pub fn new(label: &str, children: Vec<TreeNode>) -> Self {
        TreeNode {
            label: label.to_string(),
            children: children,
        }
    }
}
