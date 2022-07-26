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

fn parse_line2(line: &str) -> (usize, String) {
    let grouped_parts: Vec<String> = line
        .to_string()
        .chars()
        .group_by(|&x| x == '#')
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect();

    let count_pound_signs = grouped_parts[0].len();
    let label = grouped_parts[1].trim();
    (count_pound_signs, label.to_string())
}

#[derive(Debug)]
pub struct TreeNode2 {
    pub label: String,
    pub children: Vec<TreeNode2>,
}

impl TreeNode2 {
    pub fn from_label_str(label: &str) -> Self {
        TreeNode2 {
            label: label.to_string(),
            children: vec![],
        }
    }

    pub fn from_label(label: String) -> Self {
        TreeNode2 {
            label: label,
            children: vec![],
        }
    }

    #[cfg(test)]
    pub fn new(label: &str, children: Vec<TreeNode2>) -> Self {
        TreeNode2 {
            label: label.to_string(),
            children: children,
        }
    }
}

fn parse_markdown(content: String) -> TreeNode2 {
    let lines: Vec<&str> = content
        .split("\n")
        .map(|x| x.trim())
        .filter(|&x| !x.is_empty())
        .filter(|&x| x.starts_with("#"))
        .collect();

    let (_depth, label) = parse_line2(lines[0]);

    let root = TreeNode2::from_label(label);

    let mut stack: Vec<Vec<TreeNode2>> = vec![vec![root]];

    for line in &lines[1..] {
        let (depth, label) = parse_line2(line);

        while depth < stack.len() {
            let children = stack.pop().unwrap();
            stack.last_mut().unwrap().last_mut().unwrap().children = children;
        }

        let node = TreeNode2::from_label(label);
        if depth > stack.len() {
            stack.push(vec![node]);
        } else {
            assert_eq!(depth, stack.len());
            stack.last_mut().unwrap().push(node);
        }
    }

    while stack.len() > 1 {
        let children = stack.pop().unwrap();
        stack.last_mut().unwrap().last_mut().unwrap().children = children;
    }

    assert_eq!(stack.len(), 1);
    let mut root_layer = stack.pop().unwrap();

    assert_eq!(root_layer.len(), 1);
    root_layer.pop().unwrap()
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
    fn test_parse_line_with_with_space() {
        let actual = parse_line2("# Hello World ");
        assert_eq!(actual, (1, "Hello World".to_owned()))
    }

    #[test]
    fn test_parse_content_root() {
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

    #[test]
    fn test_parse_markdown_root() {
        let node = parse_markdown("#Root\n".to_string());

        assert_eq!(node.label, "Root");
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn test_parse_markdown_ignore_none_title_lines() {
        // Both empty lines and none-title lines are ignored
        let node = parse_markdown(
            r#"
            #Root
            hello world
            "#
            .to_string(),
        );

        assert_eq!(node.label, "Root");
        assert_eq!(node.children.len(), 0);
    }

    #[test]
    fn test_parse_markdown_root_with_children() {
        let node = parse_markdown(
            r#"
            #Root
            ##Child1
            ##Child2
            "#
            .to_string(),
        );

        assert_eq!(node.label, "Root");
        assert_eq!(node.children.len(), 2);
    }
}
