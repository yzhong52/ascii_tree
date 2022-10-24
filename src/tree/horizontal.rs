use crate::tree::tree_node::TreeNode;
use std::io;
use std::io::Write;
use std::str;

// This is the public interface to be called outside
pub fn print_nodes_std(children: &Vec<TreeNode>) {
    print_nodes(children, "", &mut io::stdout())
}

// This function allow us to write unit tests easily
fn print_nodes(children: &Vec<TreeNode>, prefix: &str, output: &mut impl Write) {
    for (idx, child) in children.iter().enumerate() {
        if idx < children.len() - 1 {
            _ = output.write_all(format!("{}├─ {}\n", prefix, child.label).as_bytes());
            print_nodes(&child.children, &format!("{}{}", prefix, "│  "), output);
        } else {
            _ = output.write_all(format!("{}└─ {}\n", prefix, child.label).as_bytes());
            print_nodes(&child.children, &format!("{}{}", prefix, "   "), output);
        }
    }
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::test_utils::assert_canonical_eq;

    #[test]
    fn test_print_single_root() {
        let mut output: Vec<u8> = Vec::new();

        print_nodes(&vec![TreeNode::new("Root", vec![])], "", &mut output);

        assert_canonical_eq(
            r#"
            └─ Root
            "#,
            str::from_utf8(&output).expect("Invalid UTF-8"),
        )
    }

    #[test]
    fn test_print_single_root_with_two_children() {
        let mut output: Vec<u8> = Vec::new();

        print_nodes(
            &vec![TreeNode::new(
                "Root",
                vec![
                    TreeNode::new("Child 1", vec![]),
                    TreeNode::new("Child 2", vec![]),
                ],
            )],
            "",
            &mut output,
        );

        assert_canonical_eq(
            r#"
            └─ Root
               ├─ Child 1
               └─ Child 2
            "#,
            str::from_utf8(&output).expect("Invalid UTF-8"),
        )
    }

    #[test]
    fn test_print_multiple_root_nodes() {
        let mut output: Vec<u8> = Vec::new();

        print_nodes(
            &vec![
                TreeNode::new("Root 1", vec![]),
                TreeNode::new("Root 2", vec![]),
            ],
            "",
            &mut output,
        );

        assert_canonical_eq(
            r#"
            ├─ Root 1
            └─ Root 2
            "#,
            str::from_utf8(&output).expect("Invalid UTF-8"),
        )
    }

    #[test]
    fn test_print_multiple_root_nodes_with_multiple_children() {
        let mut output: Vec<u8> = Vec::new();

        print_nodes(
            &vec![
                TreeNode::new(
                    "Root 1",
                    vec![
                        TreeNode::new("Child 1", vec![]),
                        TreeNode::new("Child 2", vec![]),
                    ],
                ),
                TreeNode::new(
                    "Root 2",
                    vec![
                        TreeNode::new("Child 3", vec![]),
                        TreeNode::new("Child 4", vec![]),
                    ],
                ),
            ],
            "",
            &mut output,
        );

        assert_canonical_eq(
            r#"
            ├─ Root 1
            │  ├─ Child 1
            │  └─ Child 2
            └─ Root 2
               ├─ Child 3
               └─ Child 4
            "#,
            str::from_utf8(&output).expect("Invalid UTF-8"),
        )
    }
}
