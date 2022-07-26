#[derive(Debug)]
pub struct TreeNode {
    pub label: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    #[cfg(test)]
    pub fn from_label_str(label: &str) -> Self {
        TreeNode {
            label: label.to_string(),
            children: vec![],
        }
    }

    pub fn from_label(label: String) -> Self {
        TreeNode {
            label: label,
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
