extern crate itertools;
use crate::tree::drawable::DrawableTreeNode;
use crate::tree::style::BoxDrawings;
use crate::tree::tree_node::TreeNode;
use clap::Parser;
use itertools::Itertools;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
pub mod tree;

const DEFAULT_STYLE: BoxDrawings = BoxDrawings {
    up_and_left: '┌',
    up_and_right: '┐',
    down_and_left: '└',
    down_and_right: '┘',
    vertical: '│',
    horizontal: '─',
    vertical_and_horizontal: '┼',
    down_and_horizontal: '┬',
    up_and_horizontal: '┴',
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The input filename
    #[clap(short, long, value_parser)]
    input: String,
}

fn parse(filename: String) -> Rc<RefCell<TreeNode>> {
    let contents = fs::read_to_string(filename.clone())
        .expect(format!("Fail to read input file {}", filename).as_str());

    let lines: Vec<&str> = contents.split("\n").collect();

    let root = Rc::new(RefCell::new(TreeNode {
        label: lines[0].to_string(),
        children: vec![],
    }));

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root.clone()];

    for i in 1..lines.len() {
        let line = lines[i];

        // e.g. `["#", "Child 1"]`, or `["##", "Grandchild 1"]`
        let grouped_parts: Vec<String> = line
            .to_string()
            .chars()
            .group_by(|&x| x == '#')
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect();

        let depth = grouped_parts[0].len();

        while depth < stack.len() {
            let _ = &stack.pop();
        }

        let node = TreeNode {
            label: grouped_parts[1].to_string(),
            children: vec![],
        };

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

fn main() {
    let args = Args::parse();

    let root = parse(args.input);

    let drawable_root = DrawableTreeNode::new(root.borrow());

    let mut array: Vec<Vec<char>> =
        vec![vec![' '; drawable_root.overall_width]; drawable_root.overall_height];

    drawable_root.render(&mut array, DEFAULT_STYLE);
    let result = array
        .iter()
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", result);
}
