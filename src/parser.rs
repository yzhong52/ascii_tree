extern crate itertools;
extern crate std;
use crate::tree::tree_node::TreeNode;
use itertools::Itertools;
use std::fs;

pub fn parse(filename: &String) -> Vec<TreeNode> {
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

struct NodeLayer {
    depth: usize,
    nodes: Vec<TreeNode>,
}

fn parse_markdown(content: String) -> Vec<TreeNode> {
    let lines: Vec<&str> = content
        .split("\n")
        .map(|x| x.trim())
        .filter(|&x| !x.is_empty())
        .filter(|&x| x.starts_with("#"))
        .collect();

    // Create a dummy root node at depth 0
    let root_layer = NodeLayer {
        depth: 0,
        nodes: vec![TreeNode::from_label("[DUMMY]")],
    };

    let mut stack: Vec<NodeLayer> = vec![root_layer];

    for line in &lines {
        let (depth, label) = parse_line(line);

        while depth < stack.last().unwrap().depth {
            let top_layer = stack.pop().unwrap();
            stack.last_mut().unwrap().nodes.last_mut().unwrap().children = top_layer.nodes;
        }

        let node = TreeNode::from_label(&label);
        if depth > stack.last().unwrap().depth {
            stack.push(NodeLayer {
                depth: depth,
                nodes: vec![node],
            });
        } else {
            assert_eq!(depth, stack.last().unwrap().depth);
            stack.last_mut().unwrap().nodes.push(node);
        }
    }

    while stack.len() > 1 {
        let top_layer = stack.pop().unwrap();
        stack.last_mut().unwrap().nodes.last_mut().unwrap().children = top_layer.nodes;
    }

    assert_eq!(stack.len(), 1);
    let mut root_layer = stack.pop().unwrap();

    assert_eq!(root_layer.nodes.len(), 1);
    let dummy_root = root_layer.nodes.pop().unwrap();
    dummy_root.children
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
        let nodes = parse_markdown("#Root\n".to_string());
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Root");
        assert_eq!(nodes[0].children.len(), 0);
    }

    #[test]
    fn test_parse_markdown_ignore_none_title_lines() {
        // Both empty lines and none-title lines are ignored
        let nodes = parse_markdown(
            r#"
            #Root
            hello world
            "#
            .to_string(),
        );

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Root");
        assert_eq!(nodes[0].children.len(), 0);
    }

    #[test]
    fn test_parse_markdown_root_with_children() {
        let nodes = parse_markdown(
            r#"
            #Root
            ##Child1
            ##Child2
            "#
            .to_string(),
        );

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Root");
        assert_eq!(nodes[0].children.len(), 2);
    }

    #[test]
    fn test_parse_markdown_multiple_roots() {
        let nodes = parse_markdown(
            r#"
            # Root 1
            ## Child 1.1
            ## Child 1.2
            # Root 2
            ## Child 2.1
            "#
            .to_string(),
        );

        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].label, "Root 1");
        assert_eq!(nodes[0].children.len(), 2);

        assert_eq!(nodes[1].label, "Root 2");
        assert_eq!(nodes[1].children.len(), 1);
        assert_eq!(nodes[1].children[0].label, "Child 2.1");
    }

    #[test]
    fn test_parse_invalid_children() {
        let nodes = parse_markdown(
            r#"
            # Root 1
            ### Child 1.1
            ### Child 1.2
            # Root 2
            "#
            .to_string(),
        );

        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].label, "Root 1");
        assert_eq!(nodes[0].children.len(), 2);
        assert_eq!(nodes[0].children[0].label, "Child 1.1");
        assert_eq!(nodes[0].children[1].label, "Child 1.2");
        assert_eq!(nodes[1].label, "Root 2");
    }
}
