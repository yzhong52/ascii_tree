use crate::tree::tree_node::TreeNode;

pub fn print_nodes(children: &Vec<TreeNode>, prefix: &str) {
    for (idx, child) in children.iter().enumerate() {
        if idx < children.len() - 1 {
            println!("{}├─ {}", prefix, child.label);
            print_nodes(&child.children, &format!("{}{}", prefix, "│  "));
        } else {
            println!("{}└─ {}", prefix, child.label);
            print_nodes(&child.children, &format!("{}{}", prefix, "   "));
        }
    }
}
