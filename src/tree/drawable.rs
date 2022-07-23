use crate::tree::style::BoxDrawings;
use crate::tree::tree_node::TreeNode;
use std::cell::Ref;
use std::cmp::max;

static HORIZONTAL_CHILDREN_BUFFER: usize = 2;
static VERTICAL_LAYER_BUFFER: usize = 1;

#[derive(Debug)]
struct Point2D<T> {
    pub x: T,
    pub y: T,
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

    pub fn render(&self, buffer: &mut Vec<Vec<char>>, style: BoxDrawings) {
        self.render_internal(buffer, &Point2D { x: 0, y: 0 }, &style)
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
        for i in 0..self.label.len() {
            buffer[origin.y + 1][label_start + i] = self.label.as_bytes()[i] as char
        }

        if origin.y != 0 || origin.x != 0 {
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
                //      ┌─────┴──────┐<- VERTICAL_LAYER_BUFFER
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
                        + child.width
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

                    child_origin.x += child.width + HORIZONTAL_CHILDREN_BUFFER;
                }
            }
        }
    }
}
