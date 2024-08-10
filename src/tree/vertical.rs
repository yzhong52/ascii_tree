extern crate num;

use self::num::Zero;
use crate::tree::style::BoxDrawings;
use crate::tree::tree_node::TreeNode;
use std::cmp::max;

pub fn render(tree_node: &TreeNode, style: &BoxDrawings, horizontal_spacing: usize) -> String {
    let drawble = DrawableTreeNode::new(&tree_node, horizontal_spacing);
    drawble.render(style, horizontal_spacing)
}

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

    // For multi-line label, vec!["Root", "Node"]
    // ┌──────┐
    // │ Root │
    // │ Node │
    // └──────┘
    labels: Vec<String>,

    // Children left offset
    // ┌────────────────────────┐
    // │ L1 A Very Looong Label │
    // └───────────┬────────────┘
    //       ┌─────┴───────┐
    //    ┌──┴──┐  ┌───────┴────────┐
    //    │ L2A │  │ L2B Long Label │
    //    └─────┘  └────────────────┘
    // ↑↑↑
    // The additional padding to start the children node
    chhildren_left_offset: usize,

    // A list of children
    children: Vec<DrawableTreeNode>,
}

impl DrawableTreeNode {
    pub fn new(node: &TreeNode, horizontal_spacing: usize) -> Self {
        let label = node.label.replace("\\n", "\n");
        let labels: Vec<String> = label.split('\n').map(|x| x.to_string()).collect();

        // A space on both side, and two vertical bars, i.e.:
        // ┌──────┐ <- 1
        // │ Root │
        // │ Node │
        // └──────┘ <- 2
        // ↑↑    ↑↑
        // 12    34
        let node_width = labels.iter().map(|x| x.len()).max().unwrap() + 4;
        // One horizontal bar at the top, one at the bottom
        let node_height = labels.len() + 2;

        let drawable_children: Vec<DrawableTreeNode> = node
            .children
            .iter()
            .map(|x| DrawableTreeNode::new(x, horizontal_spacing))
            .collect();

        let children_width: usize = if node.children.len() == 0 {
            0
        } else {
            // We put all the children next to each other, with some space in between
            drawable_children
                .iter()
                .map(|child| child.overall_width)
                .sum::<usize>()
                + (node.children.len() - 1) * horizontal_spacing
        };

        // The height of the current cell and all the children
        let overall_height = if node.children.len() == 0 {
            node_height
        } else {
            let children_height: usize = drawable_children
                .iter()
                .map(|child| child.overall_height)
                .max()
                .unwrap_or(0);

            if node.children.len() == 1 {
                // With single children, no vertical buffer needed.
                //     ┌──────┐
                //     │ Root │
                //     └──┬───┘
                //   ┌────┴────┐
                //   │ Child 1 │
                //   └─────────┘
                node_height + children_height
            } else {
                // More than 1 direct children, vertical buffer needed.
                //         ┌──────┐
                //         │ Root │
                //         └──┬───┘
                //      ╔═════╩══════╗<- VERTICAL_LAYER_BUFFER
                // ┌────┴────┐  ┌────┴────┐
                // │ Child 1 │  │ Child 2 │
                // └─────────┘  └─────────┘
                node_height + children_height + VERTICAL_LAYER_BUFFER
            }
        };

        //    ┌------┐
        //    │      │
        //    └------┘
        //       ↑↑
        //    01234567
        // When node_width is even (e.g. 8), we have two options (position 3 & 4 above).
        // We choose to put the center closer to the left (position 3 above).
        let center_of_current_box = (node_width - 1) / 2;

        // The overall center of the current node and all its children
        let (center_x, chhildren_left_offset, overall_width) =
            match (drawable_children.first(), drawable_children.last()) {
                (Some(first), Some(last)) => {
                    // If there are children, let's put the current node to middle of
                    // all the children.
                    let connection_bar_width = children_width
                        - (first.center_x - 1)
                        - (last.overall_width - last.center_x);

                    // When connection_bar_width is even (e.g. 8), we should add 4.
                    // When connection_bar_width is odd (e.g. 7), we should add 4.
                    let center_of_children = (first.center_x - 1) + (connection_bar_width + 1) / 2;

                    let overall_center = max(center_of_current_box, center_of_children);
                    let chhildren_left_offset =
                        max(0, center_of_current_box as i32 - center_of_children as i32) as usize;
                    let current_node_right_bufffer = node_width / 2;
                    let last_child_right_buffer = last.overall_width - last.center_x;
                    let connection_bar_right_buffer = connection_bar_width / 2;
                    let overall_width = max(
                        overall_center + current_node_right_bufffer + 1,
                        overall_center + connection_bar_right_buffer + last_child_right_buffer,
                    );

                    (overall_center, chhildren_left_offset, overall_width)
                }
                _ => (center_of_current_box, 0, node_width),
            };

        DrawableTreeNode {
            center_x: center_x,
            width: node_width,
            height: node_height,
            overall_width: overall_width,
            overall_height: overall_height,
            labels: labels,
            chhildren_left_offset: chhildren_left_offset,
            children: drawable_children,
        }
    }

    pub fn render(&self, style: &BoxDrawings, horizontal_spacing: usize) -> String {
        let mut canvas: Vec<Vec<char>> = vec![vec![' '; self.overall_width]; self.overall_height];

        self.render_internal(
            &mut canvas,
            &Point2D { x: 0, y: 0 },
            &style,
            horizontal_spacing,
        );

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
        horizontal_spacing: usize,
    ) {
        let left = origin.x + self.center_x - (self.width - 1) / 2;
        let right = left + self.width;

        // Horizontal bars
        // ┌══════┐
        // │      │
        // │      │
        // └══════┘
        for x in left + 1..right - 1 {
            buffer[origin.y][x] = style.horizontal;
            buffer[origin.y + self.height - 1][x] = style.horizontal;
        }

        // Vertical bars
        // ┌──────┐
        // ║      ║
        // ║      ║
        // └──────┘
        for y in origin.y + 1..origin.y + self.height - 1 {
            buffer[y][left] = style.vertical;
            buffer[y][right - 1] = style.vertical;
        }

        // Four corners
        // ╔──────╗
        // │      │
        // │      │
        // ╚──────╝
        buffer[origin.y][left] = style.up_and_left;
        buffer[origin.y][right - 1] = style.up_and_right;
        buffer[origin.y + self.height - 1][left] = style.down_and_left;
        buffer[origin.y + self.height - 1][right - 1] = style.down_and_right;

        // Label
        for (row_index, label) in self.labels.iter().enumerate() {
            let label_start = left + (self.width - label.len()) / 2;
            for (i, ch) in label.chars().enumerate() {
                buffer[origin.y + row_index + 1][label_start + i] = ch;
            }
        }

        // Top connection
        if origin != &Point2D::<usize>::zero() {
            buffer[origin.y][origin.x + self.center_x] =
                style.top_connection.unwrap_or(style.up_and_horizontal);
        }

        self.render_children(
            buffer,
            origin,
            style,
            horizontal_spacing,
        );
    }

    fn render_children(
        &self,
        buffer: &mut Vec<Vec<char>>,
        origin: &Point2D<usize>,
        style: &BoxDrawings,
        horizontal_spacing: usize,
    ) {
        // Draw children
        if self.children.len() != 0 {
            // Bottom connection
            buffer[origin.y + self.height - 1][origin.x + self.center_x] =
                style.bottom_connection.unwrap_or(style.down_and_horizontal);

            let child_origin_y = if self.children.len() > 1 {
                // More than 1 direct children, vertical buffer needed.
                //         ┌──────┐
                //         │ Root │
                //         └──┬───┘
                //      ╔═════╩══════╗<- VERTICAL_LAYER_BUFFER
                // ┌────┴────┐  ┌────┴────┐
                // │ Child 1 │  │ Child 2 │
                // └─────────┘  └─────────┘
                origin.y + self.height + VERTICAL_LAYER_BUFFER
            } else {
                // With single children, no vertical buffer needed.
                //     ┌──────┐
                //     │ Root │
                //     └──┬───┘
                //   ┌────┴────┐
                //   │ Child 1 │
                //   └─────────┘
                origin.y + self.height
            };

            let mut child_origin: Point2D<usize> = Point2D {
                x: origin.x + self.chhildren_left_offset,
                y: child_origin_y,
            };

            for child_id in 0..self.children.len() {
                let child = &self.children[child_id];
                child.render_internal(buffer, &child_origin, style, horizontal_spacing);

                if child_id != self.children.len() - 1 {
                    let start = child_origin.x + child.center_x + 1;
                    let end = child_origin.x
                        + child.overall_width
                        + horizontal_spacing
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

                    child_origin.x += child.overall_width + horizontal_spacing;
                }
            }
        }
    }
}

#[cfg(test)]
static HORIZONTAL_CHILDREN_SPACING: usize = 2;

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::test_utils::assert_canonical_eq;

    #[test]
    fn test_root() {
        let root = TreeNode::from_label("root");
        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);
        let expected = r#"
        ┌──────┐
        │ root │
        └──────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_one_child() {
        let child1 = TreeNode::from_label("child1");
        let root = TreeNode::new("root", vec![child1]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
         ┌──────┐
         │ root │
         └──┬───┘
        ┌───┴────┐
        │ child1 │
        └────────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_two_children() {
        let child1 = TreeNode::from_label("child1");
        let child2 = TreeNode::from_label("child2");
        let root = TreeNode::new("root", vec![child1, child2]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
               ┌──────┐
               │ root │
               └──┬───┘
            ┌─────┴─────┐
        ┌───┴────┐  ┌───┴────┐
        │ child1 │  │ child2 │
        └────────┘  └────────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_three_children() {
        let child1 = TreeNode::from_label("child1");
        let child2 = TreeNode::from_label("child2");
        let child3 = TreeNode::from_label("child3");
        let root = TreeNode::new("root", vec![child1, child2, child3]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
                     ┌──────┐
                     │ root │
                     └──┬───┘
            ┌───────────┼───────────┐
        ┌───┴────┐  ┌───┴────┐  ┌───┴────┐
        │ child1 │  │ child2 │  │ child3 │
        └────────┘  └────────┘  └────────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_root_with_grandchildren() {
        let grandchild1 = TreeNode::from_label("grandchild1");
        let grandchild2 = TreeNode::from_label("grandchild2");
        let grandchild3 = TreeNode::from_label("grandchild3");

        let child1 = TreeNode::new("child1", vec![grandchild1, grandchild2]);
        let child2 = TreeNode::new("child2", vec![grandchild3]);

        let root = TreeNode::new("root", vec![child1, child2]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

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
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_multi_line_label() {
        let root = TreeNode::from_label("Root\\nNode");
        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);
        let expected = r#"
        ┌──────┐
        │ Root │
        │ Node │
        └──────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_multi_line_label_with_grandchildren() {
        let grandchild1 = TreeNode::from_label("grandchild1\\nnode");
        let grandchild2 = TreeNode::from_label("grandchild2\\nnode");
        let grandchild3 = TreeNode::from_label("grandchild3\\nnode");

        let child1 = TreeNode::new("child1\\nnode", vec![grandchild1, grandchild2]);
        let child2 = TreeNode::new("child2\\nnode", vec![grandchild3]);

        let root = TreeNode::new("root\\nnode", vec![child1, child2]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
                                 ┌──────┐
                                 │ root │
                                 │ node │
                                 └──┬───┘
                       ┌────────────┴────────────┐
                   ┌───┴────┐                ┌───┴────┐
                   │ child1 │                │ child2 │
                   │  node  │                │  node  │
                   └───┬────┘                └───┬────┘
               ┌───────┴────────┐         ┌──────┴──────┐
        ┌──────┴──────┐  ┌──────┴──────┐  │ grandchild3 │
        │ grandchild1 │  │ grandchild2 │  │    node     │
        │    node     │  │    node     │  └─────────────┘
        └─────────────┘  └─────────────┘                 "#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_parent_wider_than_children() {
        let child1 = TreeNode::from_label("child");
        let root = TreeNode::new("a long root node", vec![child1]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
        ┌──────────────────┐
        │ a long root node │
        └────────┬─────────┘
             ┌───┴───┐
             │ child │
             └───────┘      "#;
        assert_canonical_eq(&result, &expected);

        let children = vec![TreeNode::from_label("a"), TreeNode::from_label("b")];
        let root = TreeNode::new("a long root node", children);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
        ┌──────────────────┐
        │ a long root node │
        └────────┬─────────┘
              ┌──┴───┐
            ┌─┴─┐  ┌─┴─┐
            │ a │  │ b │
            └───┘  └───┘    "#;
        assert_canonical_eq(&result, &expected);

        let children = vec![
            TreeNode::from_label("a"),
            TreeNode::from_label("b"),
            TreeNode::from_label("c"),
        ];
        let root = TreeNode::new("a long root node", children);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
        ┌──────────────────┐
        │ a long root node │
        └────────┬─────────┘
          ┌──────┼──────┐
        ┌─┴─┐  ┌─┴─┐  ┌─┴─┐
        │ a │  │ b │  │ c │
        └───┘  └───┘  └───┘ "#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_inbalance_children_1() {
        let child_l2a = TreeNode::from_label("L2A");
        let child_l2b = TreeNode::from_label("L2B Long Label");
        let root = TreeNode::new("L1 A Very Looong Label", vec![child_l2a, child_l2b]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
        ┌────────────────────────┐   
        │ L1 A Very Looong Label │   
        └───────────┬────────────┘   
             ┌──────┴──────┐         
          ┌──┴──┐  ┌───────┴────────┐
          │ L2A │  │ L2B Long Label │
          └─────┘  └────────────────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[test]
    fn test_inbalance_children_2() {
        let child_l2a = TreeNode::from_label("L2A Long Label");
        let child_l2b = TreeNode::from_label("L2B");
        let root = TreeNode::new("L1 A Very Looong Label", vec![child_l2a, child_l2b]);

        let drawable_root = DrawableTreeNode::new(&root, HORIZONTAL_CHILDREN_SPACING);
        let result = drawable_root.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);

        let expected = r#"
       ┌────────────────────────┐   
       │ L1 A Very Looong Label │   
       └───────────┬────────────┘   
            ┌──────┴───────┐        
    ┌───────┴────────┐  ┌──┴──┐     
    │ L2A Long Label │  │ L2B │     
    └────────────────┘  └─────┘"#;
        assert_canonical_eq(&result, &expected);
    }
}

#[cfg(test)]
mod style_tests {
    extern crate rstest;

    use self::rstest::*;
    use super::*;
    use crate::test_utils::assert_canonical_eq;

    #[fixture]
    pub fn drawable() -> DrawableTreeNode {
        let child1: TreeNode = TreeNode::from_label("child1");
        let child2: TreeNode = TreeNode::from_label("child2");
        let root: TreeNode = TreeNode::new("root", vec![child1, child2]);
        DrawableTreeNode::new(&root, 2)
    }

    #[rstest]
    fn test_style_thin(drawable: DrawableTreeNode) {
        let result = drawable.render(&BoxDrawings::THIN, HORIZONTAL_CHILDREN_SPACING);
        let expected = r#"
               ┌──────┐
               │ root │
               └──┬───┘
            ┌─────┴─────┐
        ┌───┴────┐  ┌───┴────┐
        │ child1 │  │ child2 │
        └────────┘  └────────┘"#;
        assert_canonical_eq(&result, &expected);
    }

    #[rstest]
    fn test_style_thick(drawable: DrawableTreeNode) {
        let result = drawable.render(&BoxDrawings::THICK, HORIZONTAL_CHILDREN_SPACING);
        let expected = r#"
               ┏━━━━━━┓
               ┃ root ┃
               ┗━━┳━━━┛
            ┏━━━━━┻━━━━━┓
        ┏━━━┻━━━━┓  ┏━━━┻━━━━┓
        ┃ child1 ┃  ┃ child2 ┃
        ┗━━━━━━━━┛  ┗━━━━━━━━┛"#;
        assert_canonical_eq(&result, &expected);
    }

    #[rstest]
    fn test_style_double(drawable: DrawableTreeNode) {
        let result = drawable.render(&BoxDrawings::DOUBLE, HORIZONTAL_CHILDREN_SPACING);
        let expected = r#"
               ╔══════╗
               ║ root ║
               ╚══╦═══╝
            ╔═════╩═════╗
        ╔═══╩════╗  ╔═══╩════╗
        ║ child1 ║  ║ child2 ║
        ╚════════╝  ╚════════╝"#;
        assert_canonical_eq(&result, &expected);
    }

    #[rstest]
    fn test_style_with_top_connection(drawable: DrawableTreeNode) {
        let result = drawable.render(
            &BoxDrawings {
                up_and_left: '┌',
                up_and_right: '┐',
                down_and_left: '└',
                down_and_right: '┘',
                vertical: '│',
                horizontal: '─',
                vertical_and_horizontal: '┼',
                down_and_horizontal: '┬',
                up_and_horizontal: '┴',
                top_connection: Some('▼'),
                bottom_connection: None,
            },
            HORIZONTAL_CHILDREN_SPACING,
        );
        let expected = r#"
               ┌──────┐
               │ root │
               └──┬───┘
            ┌─────┴─────┐
        ┌───▼────┐  ┌───▼────┐
        │ child1 │  │ child2 │
        └────────┘  └────────┘"#;
        assert_canonical_eq(&result, &expected);
    }
}
