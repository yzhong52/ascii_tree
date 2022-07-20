use std::cmp::max;

struct BoxDrawings {
    up_and_left: char,
    up_and_right: char,
    down_and_left: char,
    down_and_right: char,
    vertical: char,
    horizontal: char,
    vertical_and_horizontal: char,
    down_and_horizontal: char,
    up_and_horizontal: char,
}

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

#[derive(Debug)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}
#[derive(Debug)]
struct TreeNode {
    label: String,
    children: Vec<TreeNode>,
}
#[derive(Debug)]
struct DrawableTreeNode {
    // Horizontal center of the current node
    center_x: usize,
    // Size of the node
    width: usize,
    height: usize,
    // Size of the node with all its children (if any)
    overall_width: usize,
    overall_height: usize,
    // TODO: Yuchen - add support for multi-line labels with '\n'
    label: String,
    children: Vec<DrawableTreeNode>,
}

static HORIZONTAL_CHILDREN_BUFFER: usize = 2;
static VERTICAL_LAYER_BUFFER: usize = 1;

impl TreeNode {
    fn to_drawable(&self) -> DrawableTreeNode {
        let drawable_children: Vec<DrawableTreeNode> =
            self.children.iter().map(|x| x.to_drawable()).collect();

        let children_width: usize = if self.children.len() == 0 {
            0
        } else {
            // We put all the children next to each other, with some space (HORIZONTAL_CHILDREN_BUFFER)
            // in between
            drawable_children
                .iter()
                .map(|child| child.width)
                .sum::<usize>()
                + (self.children.len() - 1) * HORIZONTAL_CHILDREN_BUFFER
        };

        let children_height: usize = drawable_children
            .iter()
            .map(|child| child.height)
            .max()
            .unwrap_or(0);

        // A space on both side, and two vertical bars, i.e.:
        // ┌──────┐
        // │ Root │
        // └──────┘
        // ^^    ^^
        // 12    34
        let node_width = self.label.len() + 4;
        let node_height = 3;

        let overall_width = max(node_width, children_width);
        let overall_height = if children_height == 0 {
            node_height
        } else {
            node_height + children_height + VERTICAL_LAYER_BUFFER
        };

        let center_x = match (drawable_children.first(), drawable_children.last()) {
            (Some(first), Some(last)) => {
                // If there are children, let's put the current node to middle of
                // all the children.
                (first.center_x + children_width - last.width + last.center_x) / 2
            }
            _ => {
                //    ┌--┴---┐
                //    │      │
                //    └--┳┳--┘
                //    012345678
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
            label: self.label.clone(),
            children: drawable_children,
        }
    }
}
impl DrawableTreeNode {
    fn render(&self, buffer: &mut Vec<Vec<char>>, origin: &Point2D<usize>) {
        let left = origin.x + self.center_x - (self.width - 1) / 2;
        let right = left + self.width;

        for x in left + 1..right - 1 {
            buffer[origin.y][x] = DEFAULT_STYLE.horizontal;
            buffer[origin.y + self.height - 1][x] = DEFAULT_STYLE.horizontal;
        }

        for y in origin.y + 1..origin.y + self.height - 1 {
            buffer[y][left] = DEFAULT_STYLE.vertical;
            buffer[y][right - 1] = DEFAULT_STYLE.vertical;
        }

        buffer[origin.y][left] = DEFAULT_STYLE.up_and_left;
        buffer[origin.y][right - 1] = DEFAULT_STYLE.up_and_right;
        buffer[origin.y + self.height - 1][left] = DEFAULT_STYLE.down_and_left;
        buffer[origin.y + self.height - 1][right - 1] = DEFAULT_STYLE.down_and_right;

        let label_start = left + 2;
        for i in 0..self.label.len() {
            buffer[origin.y + 1][label_start + i] = self.label.as_bytes()[i] as char
        }

        if origin.y != 0 || origin.x != 0 {
            buffer[origin.y][origin.x + self.center_x] = DEFAULT_STYLE.up_and_horizontal;
        }
        if self.children.len() != 0 {
            buffer[origin.y + self.height - 1][origin.x + self.center_x] =
                DEFAULT_STYLE.down_and_horizontal;

            let mut child_origin = Point2D {
                x: origin.x,
                y: origin.y + self.height + VERTICAL_LAYER_BUFFER,
            };

            for child_id in 0..self.children.len() {
                let child = &self.children[child_id];
                child.render(buffer, &child_origin);

                if child_id != self.children.len() - 1 {
                    let start = child_origin.x + child.center_x + 1;
                    let end = child_origin.x
                        + child.width
                        + HORIZONTAL_CHILDREN_BUFFER
                        + self.children[child_id + 1].center_x;
                    for x in start..end {
                        if x != origin.x + self.center_x {
                            buffer[origin.y + self.height][x] = DEFAULT_STYLE.horizontal;
                        } else {
                            //         ┌──────┐
                            //         │ Root │
                            //         └──┬───┘
                            //      ┌─────╩──────┐
                            // ┌────┴────┐↑ ┌────┴────┐
                            // │ Child 1 │  │ Child 2 │
                            // └─────────┘  └─────────┘
                            buffer[origin.y + self.height][x] = DEFAULT_STYLE.up_and_horizontal;
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
                        buffer[origin.y + self.height][start - 1] = DEFAULT_STYLE.up_and_left;
                    }

                    if child_id == self.children.len() - 2 {
                        //         ┌──────┐
                        //         │ Root │
                        //         └──┬───┘
                        //      ┌─────┴──────╗<-
                        // ┌────┴────┐  ┌────┴────┐
                        // │ Child 1 │  │ Child 2 │
                        // └─────────┘  └─────────┘
                        buffer[origin.y + self.height][end] = DEFAULT_STYLE.up_and_right;
                    } else if end == origin.x + self.center_x {
                        //                 ┌──────┐
                        //                 │ Root │
                        //                 └──┬───┘
                        //       ┌────────────╬────────────┐
                        //  ┌────┴────┐  ┌────┴────┐  ┌────┴────┐
                        //  │ Child 1 │  │ Child 2 │  │ Child 3 │
                        //  └─────────┘  └─────────┘  └─────────┘
                        buffer[origin.y + self.height][end] = DEFAULT_STYLE.vertical_and_horizontal;
                    } else {
                        //                         ┌──────┐
                        //                         │ Root │
                        //                      ↓  └──┬───┘  ↓
                        //         ┌────────────╦─────┴──────╦────────────┐
                        //    ┌────┴────┐  ┌────┴────┐  ┌────┴────┐  ┌────┴────┐
                        //    │ Child 1 │  │ Child 2 │  │ Child 3 │  │ Child 4 │
                        //    └─────────┘  └─────────┘  └─────────┘  └─────────┘
                        buffer[origin.y + self.height][end] = DEFAULT_STYLE.down_and_horizontal;
                    }

                    child_origin.x += child.width + HORIZONTAL_CHILDREN_BUFFER;
                }
            }
        }
    }
}

fn main() {
    let child1 = TreeNode {
        label: "Child 1".to_string(),
        children: Vec::new(),
    };

    let child2 = TreeNode {
        label: "Child 2".to_string(),
        children: Vec::new(),
    };

    let root = TreeNode {
        label: "Root".to_string(),
        children: vec![child1, child2],
    };

    let drawable_root = root.to_drawable();

    let mut array: Vec<Vec<char>> =
        vec![vec![' '; drawable_root.overall_width]; drawable_root.overall_height];

    drawable_root.render(&mut array, &Point2D { x: 0, y: 0 });
    let result = array
        .iter()
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", result);
}
