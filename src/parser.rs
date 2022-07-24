extern crate itertools;
extern crate std;
use crate::tree::tree_node::TreeNode;
use itertools::Itertools;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn parse(filename: String) -> Rc<RefCell<TreeNode>> {
    let content: String = fs::read_to_string(filename.clone())
        .expect(format!("Fail to read input file {}", filename).as_str());

    parse_content(content)
}

// Given a single line, return the depth of the node,
// and the corresponding content. Root node is considered
// as depth 0.
//
// #Root -> (0, "Root")
// ##Child -> (1, "Child")
fn parse_line(line: &str) -> (usize, String) {
    let grouped_parts: Vec<String> = line
        .to_string()
        .chars()
        .group_by(|&x| x == '#')
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect();

    (grouped_parts[0].len() - 1, grouped_parts[1].clone())
}

fn parse_content(content: String) -> Rc<RefCell<TreeNode>> {
    let lines: Vec<&str> = content.split("\n").filter(|&x| !x.is_empty()).collect();

    let (_depth, label) = parse_line(lines[0]);

    let root = Rc::new(RefCell::new(TreeNode::from_label(&label)));

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root.clone()];

    for line in &lines[1..] {

        let (depth, label) = parse_line(line);

        while depth < stack.len() {
            let _ = &stack.pop();
        }

        let node = TreeNode::from_label(label.as_str());

        let new_child = Rc::new(RefCell::new(node));

        stack
            .last()
            .unwrap()
            .borrow_mut()
            .children
            .push(new_child.clone());
        stack.push(new_child);
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let actual = parse_line("#Root");
        assert_eq!(actual, (0, "Root".to_owned()))
    }

    #[test]
    fn test_parse_content_root_only() {
        let node = parse_content("#Root\n".to_string());

        assert_eq!(node.borrow().label, "Root");
        assert_eq!(node.borrow().children.len(), 0);

    }

    #[test]
    fn test_parse_content_root_with_children() {
        let node = parse_content("#Root\n##Child1".to_string());

        assert_eq!(node.borrow().label, "Root");
        assert_eq!(node.borrow().children.len(), 1);
    }
}
