extern crate itertools;
extern crate std;
use crate::tree::tree_node::TreeNode;
use itertools::Itertools;
use std::fs;

pub fn parse(filename_or_content: &String, width: Option<usize>) -> Vec<TreeNode> {
    let content: String = match fs::read_to_string(filename_or_content.clone()) {
        Ok(file_content) => file_content,
        Err(_) => {
            // Cannot read as file, treat this as content instead
            filename_or_content.clone()
        }
    };

    parse_markdown(content, width)
}

fn wrap_line(line: &str, width: Option<usize>) -> String {
    match width {
        Some(w) => {
            let words = line.split(' ').collect_vec();
            let mut lines = vec![];
            let mut current = vec![];
            let mut current_width = 0;
            for word in words {
                let next_width = if current_width > 0 {
                    current_width + word.len() + 1
                } else {
                    word.len()
                };

                if next_width >= w && current_width > 0 {
                    lines.push(current.join(" "));
                    current = vec![word];
                    current_width = word.len();
                } else {
                    current.push(word);
                    current_width = next_width;
                }
            }
            lines.push(current.join(" "));
            lines.join("\n")
        }
        None => line.to_string(),
    }
}

// Given a single line, return the depth of the node,
// and the corresponding content. Root node is considered
// as depth 0.
//
// #Root -> (0, "Root")
// ##Child -> (1, "Child")
fn parse_line(line: &str, width: Option<usize>) -> (usize, String) {
    let grouped_parts: Vec<String> = line
        .to_string()
        .chars()
        .group_by(|&x| x == '#')
        .into_iter()
        .map(|(_, r)| r.collect())
        .collect();

    let count_pound_signs = grouped_parts[0].len();
    let label = wrap_line(grouped_parts[1].trim(), width);
    (count_pound_signs, label)
}

struct NodeLayer {
    depth: usize,
    nodes: Vec<TreeNode>,
}

fn parse_markdown(content: String, width: Option<usize>) -> Vec<TreeNode> {
    // Split the content by line, and remove empty lines
    let lines: Vec<&str> = content
        .split("\n")
        .map(|x| x.trim())
        .filter(|&x| !x.is_empty())
        .collect();

    // Create a dummy root node at depth 0
    let root_layer = NodeLayer {
        depth: 0,
        nodes: vec![TreeNode::from_label("[DUMMY]")],
    };

    let mut stack: Vec<NodeLayer> = vec![root_layer];

    for line in &lines {
        if line.starts_with("#") {
            let (depth, label) = parse_line(line, width);
            while depth < stack.last().unwrap().depth {
                // Finish parsing one layer of nodes
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
        } else {
            // if this line is not a title line, then append it to the last node's
            // label with a line break.
            stack
                .last_mut()
                .unwrap()
                .nodes
                .last_mut()
                .unwrap()
                .label
                .push_str(&("\\n".to_string() + line));
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
        let actual = parse_line("#Root", None);
        assert_eq!(actual, (1, "Root".to_owned()))
    }

    #[test]
    fn test_parse_line_with_with_space() {
        let actual = parse_line("# Hello World ", None);
        assert_eq!(actual, (1, "Hello World".to_owned()))
    }

    #[test]
    fn test_parse_markdown_root() {
        let nodes = parse_markdown("#Root\n".to_string(), None);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Root");
        assert_eq!(nodes[0].children.len(), 0);
    }

    #[test]
    fn test_parse_include_non_title_line() {
        // In other examples, we only parse the title lines of the markdown format.
        // Here, we parse the content as well and automatically include line breaks.
        // This, we believe is slight more usessssr friendly than having to type '\n'.
        let nodes = parse_markdown(
            r#"
            #Root
            hello world
            "#
            .to_string(),
            None,
        );

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].label, "Root\\nhello world");
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
            None,
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
            None,
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
            None,
        );

        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].label, "Root 1");
        assert_eq!(nodes[0].children.len(), 2);
        assert_eq!(nodes[0].children[0].label, "Child 1.1");
        assert_eq!(nodes[0].children[1].label, "Child 1.2");
        assert_eq!(nodes[1].label, "Root 2");
    }

    #[test]
    fn test_wrap_line() {
        assert_eq!(wrap_line("boxes and lines", Some(10)), "boxes and\nlines");
    }

    #[test]
    fn test_wrap_line_long() {
        assert_eq!(wrap_line("a-very-long-word", Some(5)), "a-very-long-word");
    }
}
