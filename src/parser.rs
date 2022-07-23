extern crate itertools;
extern crate std;
use crate::tree::tree_node::TreeNode;
use itertools::Itertools;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn parse(filename: String) -> Rc<RefCell<TreeNode>> {
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
