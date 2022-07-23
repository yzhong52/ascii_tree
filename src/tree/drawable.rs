use crate::tree::style::BoxDrawings;
use crate::tree::tree_node::TreeNode;
extern crate num;
use num::Zero;
use std::cell::Ref;
use std::cmp::max;

static HORIZONTAL_CHILDREN_BUFFER: usize = 2;
static VERTICAL_LAYER_BUFFER: usize = 1;

#[derive(Debug, Eq, PartialEq)]
struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Zero,
    T: Eq,
    T: PartialEq,
{
    pub fn zero() -> Point2D<T> {
        Point2D {
            x: Zero::zero(),
            y: Zero::zero(),
        }
    }
}

#[derive(Debug)]
pub struct DrawableTreeNode {
    // Horizontal center of the current node
    center_x: usize,
    // Size of the node
    width: usize,
    height: usize,
    // Size of the node with all its children (if any)
    pub overall_width: usize,
    pub overall_height: usize,
    // TODO: Yuchen - add support for multi-line labels with '\n'
    label: String,
    children: Vec<DrawableTreeNode>,
}

impl DrawableTreeNode {
    pub fn new(node: Ref<TreeNode>) -> Self {
        // A space on both side, and two vertical bars, i.e.:
        // ┌──────┐
        // │ Root │
        // └──────┘
        // ↑↑    ↑↑
        // 12    34
        let node_width = node.label.len() + 4;
        let node_height = 3;

        let drawable_children: Vec<DrawableTreeNode> = node
            .children
            .iter()
            .map(|x| DrawableTreeNode::new(x.borrow()))
            .collect();

        let children_width: usize = if node.children.len() == 0 {
            0
        } else {
            // We put all the children next to each other, with some space in between
            drawable_children
                .iter()
                .map(|child| child.overall_width)
                .sum::<usize>()
                + (node.children.len() - 1) * HORIZONTAL_CHILDREN_BUFFER
        };

        let overall_width = max(node_width, children_width);
        let overall_height = if node.children.len() == 0 {
            node_height
        } else {
            let children_height: usize = drawable_children
                .iter()
                .map(|child| child.overall_height)
                .max()
                .unwrap_or(0);

            if node.children.len() == 1 {
                node_height + children_height
            } else {
                node_height + children_height + VERTICAL_LAYER_BUFFER
            }
        };

        let center_x = match (drawable_children.first(), drawable_children.last()) {
            (Some(first), Some(last)) => {
                // If there are children, let's put the current node to middle of
                // all the children.
                (first.center_x + children_width - last.overall_width + last.center_x) / 2
            }
            _ => {
                //    ┌------┐
                //    │      │
                //    └------┘
                //       ↑↑
                //    01234567
                // When node_width is even (e.g. 8), we have two options (position 3 & 4 above).
                // We choose to put the center closer to the left (position 3 above).
                (node_width - 1) / 2
            }
        };

        DrawableTreeNode {
            center_x: center_x,
            width: node_width,
            height: node_height,
            overall_width: overall_width,
            overall_height: overall_height,
            label: node.label.clone(),
            children: drawable_children,
        }
    }

    pub fn render(&self, style: &BoxDrawings) -> String {
        let mut canvas: Vec<Vec<char>> = vec![vec![' '; self.overall_width]; self.overall_height];

        self.render_internal(&mut canvas, &Point2D { x: 0, y: 0 }, &style);

        canvas
            .iter()
            .map(|row| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn render_internal(
        &self,
        buffer: &mut Vec<Vec<char>>,
        origin: &Point2D<usize>,
        style: &BoxDrawings,
    ) {
        let left = origin.x + self.center_x - (self.width - 1) / 2;
        let right = left + self.width;

        for x in left + 1..right - 1 {
            buffer[origin.y][x] = style.horizontal;
            buffer[origin.y + self.height - 1][x] = style.horizontal;
        }

        for y in origin.y + 1..origin.y + self.height - 1 {
            buffer[y][left] = style.vertical;
            buffer[y][right - 1] = style.vertical;
        }

        buffer[origin.y][left] = style.up_and_left;
        buffer[origin.y][right - 1] = style.up_and_right;
        buffer[origin.y + self.height - 1][left] = style.down_and_left;
        buffer[origin.y + self.height - 1][right - 1] = style.down_and_right;

        let label_start = left + 2;
        for (i, ch) in self.label.chars().enumerate() {
            buffer[origin.y + 1][label_start + i] = ch;
        }

        if origin != &Point2D::<usize>::zero() {
            buffer[origin.y][origin.x + self.center_x] = style.up_and_horizontal;
        }
        if self.children.len() != 0 {
            buffer[origin.y + self.height - 1][origin.x + self.center_x] =
                style.down_and_horizontal;

            let mut child_origin: Point2D<usize>;

            if self.children.len() > 1 {
                // More than 1 direct children, vertical buffer needed.
                //         ┌──────┐
                //         │ Root │
                //         └──┬───┘
                //      ╔═════╩══════╗<- VERTICAL_LAYER_BUFFER
                // ┌────┴────┐  ┌────┴────┐
                // │ Child 1 │  │ Child 2 │
                // └─────────┘  └─────────┘
                child_origin = Point2D {
                    x: origin.x,
                    y: origin.y + self.height + VERTICAL_LAYER_BUFFER,
                };
            } else {
                // With single children, no vertical buffer needed.
                //     ┌──────┐
                //     │ Root │
                //     └──┬───┘
                //   ┌────┴────┐
                //   │ Child 1 │
                //   └─────────┘
                child_origin = Point2D {
                    x: origin.x,
                    y: origin.y + self.height,
                };
            }

            for child_id in 0..self.children.len() {
                let child = &self.children[child_id];
                child.render_internal(buffer, &child_origin, style);

                if child_id != self.children.len() - 1 {
                    let start = child_origin.x + child.center_x + 1;
                    let end = child_origin.x
                        + child.overall_width
                        + HORIZONTAL_CHILDREN_BUFFER
                        + self.children[child_id + 1].center_x;
                    for x in start..end {
                        if x != origin.x + self.center_x {
                            buffer[origin.y + self.height][x] = style.horizontal;
                        } else {
                            //         ┌──────┐
                            //         │ Root │
                            //         └──┬───┘
                            //      ┌─────╩──────┐
                            // ┌────┴────┐↑ ┌────┴────┐
                            // │ Child 1 │  │ Child 2 │
                            // └─────────┘  └─────────┘
                            buffer[origin.y + self.height][x] = style.up_and_horizontal;
                        }
                    }
                    if child_id == 0 {
                        //         ┌──────┐
                        //         │ Root │
                        //         └──┬───┘
                        //    ->╔─────┴──────┐
                        // ┌────┴────┐  ┌────┴────┐
                        // │ Child 1 │  │ Child 2 │
                        // └─────────┘  └─────────┘
                        buffer[origin.y + self.height][start - 1] = style.up_and_left;
                    }

                    if child_id == self.children.len() - 2 {
                        //         ┌──────┐
                        //         │ Root │
                        //         └──┬───┘
                        //      ┌─────┴──────╗<-
                        // ┌────┴────┐  ┌────┴────┐
                        // │ Child 1 │  │ Child 2 │
                        // └─────────┘  └─────────┘
                        buffer[origin.y + self.height][end] = style.up_and_right;
                    } else if end == origin.x + self.center_x {
                        //                 ┌──────┐
                        //                 │ Root │
                        //                 └──┬───┘
                        //       ┌────────────╬────────────┐
                        //  ┌────┴────┐  ┌────┴────┐  ┌────┴────┐
                        //  │ Child 1 │  │ Child 2 │  │ Child 3 │
                        //  └─────────┘  └─────────┘  └─────────┘
                        buffer[origin.y + self.height][end] = style.vertical_and_horizontal;
                    } else {
                        //                         ┌──────┐
                        //                         │ Root │
                        //                      ↓  └──┬───┘  ↓
                        //         ┌────────────╦─────┴──────╦────────────┐
                        //    ┌────┴────┐  ┌────┴────┐  ┌────┴────┐  ┌────┴────┐
                        //    │ Child 1 │  │ Child 2 │  │ Child 3 │  │ Child 4 │
                        //    └─────────┘  └─────────┘  └─────────┘  └─────────┘
                        buffer[origin.y + self.height][end] = style.down_and_horizontal;
                    }

                    child_origin.x += child.overall_width + HORIZONTAL_CHILDREN_BUFFER;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn assert_eq(input_left: &String, input_right: &str) {
        let left_rows = input_left.split('\n').collect::<Vec<&str>>();
        let right_rows = input_right
            .split('\n')
            .filter(|line| line.len() != 0)
            .collect::<Vec<&str>>();

        let extra_leading = right_rows
            .iter()
            .map(|line| {
                let mut count = 0;
                for ch in line.chars() {
                    if ch != ' ' {
                        return count;
                    }
                    count += 1;
                }

                line.len()
            })
            .min()
            .unwrap();

        assert_eq!(
            left_rows.len(),
            right_rows.len(),
            "Left:\n{}\nRight:\n{}\n",
            input_left,
            input_right
        );

        let row_by_row_comparison: String = left_rows
            .iter()
            .zip(right_rows.iter())
            .enumerate()
            .map(|(index, row)| {
                let (left_row, right_row) = row;
                format!("{:5}: {}|{}", index, left_row, &right_row[extra_leading..])
            })
            .collect::<Vec<String>>()
            .join("\n");

        for row in 0..left_rows.len() {
            assert_eq!(
                left_rows[row],
                &right_rows[row][extra_leading..],
                "\nDiffer from row {}:\n{}\nLeft:\n{}\nRight:\n{}",
                row,
                row_by_row_comparison,
                input_left,
                input_right
            );
        }
    }

    #[test]
    fn test_single_root() {
        let root = TreeNode::from_label("root");
        let drawable_root = DrawableTreeNode::new(RefCell::new(root).borrow());
        let result = drawable_root.render(&BoxDrawings::THIN);
        let expected = r#"
        ┌──────┐
        │ root │
        └──────┘"#;
        assert_eq(&result, &expected);
    }

    #[test]
    fn test_root_plus_one_child() {
        let child1 = TreeNode::from_label("child1");
        let root = TreeNode::new("root", vec![child1]);

        let drawable_root = DrawableTreeNode::new(RefCell::new(root).borrow());
        let result = drawable_root.render(&BoxDrawings::THIN);

        let expected = r#"
         ┌──────┐ 
         │ root │ 
         └──┬───┘ 
        ┌───┴────┐
        │ child1 │
        └────────┘"#;
        assert_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_two_children() {
        let child1 = TreeNode::from_label("child1");
        let child2 = TreeNode::from_label("child2");
        let root = TreeNode::new("root", vec![child1, child2]);

        let drawable_root = DrawableTreeNode::new(RefCell::new(root).borrow());
        let result = drawable_root.render(&BoxDrawings::THIN);

        let expected = r#"
                ┌──────┐       
                │ root │       
                └──┬───┘       
             ┌─────┴─────┐     
         ┌───┴────┐  ┌───┴────┐
         │ child1 │  │ child2 │
         └────────┘  └────────┘"#;
        assert_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_three_children() {
        let child1 = TreeNode::from_label("child1");
        let child2 = TreeNode::from_label("child2");
        let child3 = TreeNode::from_label("child3");
        let root = TreeNode::new("root", vec![child1, child2, child3]);

        let drawable_root = DrawableTreeNode::new(RefCell::new(root).borrow());
        let result = drawable_root.render(&BoxDrawings::THIN);

        let expected = r#"
                     ┌──────┐             
                     │ root │             
                     └──┬───┘             
            ┌───────────┼───────────┐     
        ┌───┴────┐  ┌───┴────┐  ┌───┴────┐
        │ child1 │  │ child2 │  │ child3 │
        └────────┘  └────────┘  └────────┘"#;
        assert_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_grandchildren() {
        let grandchild1 = TreeNode::from_label("grandchild1");
        let grandchild2 = TreeNode::from_label("grandchild2");
        let grandchild3 = TreeNode::from_label("grandchild3");

        let child1 = TreeNode::new("child1", vec![grandchild1, grandchild2]);
        let child2 = TreeNode::new("child2", vec![grandchild3]);

        let root = TreeNode::new("root", vec![child1, child2]);

        let drawable_root = DrawableTreeNode::new(RefCell::new(root).borrow());
        let result = drawable_root.render(&BoxDrawings::THIN);

        let expected = r#"
                                 ┌──────┐                
                                 │ root │                
                                 └──┬───┘                
                       ┌────────────┴────────────┐       
                   ┌───┴────┐                ┌───┴────┐  
                   │ child1 │                │ child2 │  
                   └───┬────┘                └───┬────┘  
               ┌───────┴────────┐         ┌──────┴──────┐
        ┌──────┴──────┐  ┌──────┴──────┐  │ grandchild3 │
        │ grandchild1 │  │ grandchild2 │  └─────────────┘
        └─────────────┘  └─────────────┘                 "#;
        assert_eq(&result, &expected);
    }
}
