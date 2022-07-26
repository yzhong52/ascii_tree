extern crate itertools;
extern crate std;
use crate::tree::tree_node::TreeNode;
use itertools::Itertools;
use std::fs;

pub fn parse(filename: String) -> TreeNode {
    let content: String = fs::read_to_string(filename.clone())
        .expect(format!("Fail to read input file {}", filename).as_str());

    parse_markdown(content)
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

    let count_pound_signs = grouped_parts[0].len();
    let label = grouped_parts[1].trim();
    (count_pound_signs, label.to_string())
}

fn parse_markdown(content: String) -> TreeNode {
    let lines: Vec<&str> = content
        .split("\n")
        .map(|x| x.trim())
        .filter(|&x| !x.is_empty())
        .filter(|&x| x.starts_with("#"))
        .collect();

    let (_depth, label) = parse_line(lines[0]);

    let root = TreeNode::from_label(label);

    let mut stack: Vec<Vec<TreeNode>> = vec![vec![root]];

    for line in &lines[1..] {
        let (depth, label) = parse_line(line);

        while depth < stack.len() {
            let children = stack.pop().unwrap();
            stack.last_mut().unwrap().last_mut().unwrap().children = children;
        }

        let node = TreeNode::from_label(label);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let actual = parse_line("#Root");
        assert_eq!(actual, (1, "Root".to_owned()))
    }
    
    #[test]
    fn test_parse_line_with_with_space() {
        let actual = parse_line("# Hello World ");
        assert_eq!(actual, (1, "Hello World".to_owned()))
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
