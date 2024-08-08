extern crate clap;
extern crate itertools;

use crate::parser::parse;

use crate::tree::style::BoxDrawings;
use crate::tree::style::Style;
use crate::tree::vertical::render;
use clap::{Parser, Subcommand};
use tree::horizontal;

mod parser;
mod test_utils;
mod tree;

const LONG_ABOUT: &str = r#"
A command line tool for drawing tree structures with ascii characters.

Example:

astree horizontal -i "$(cat << 'EOF'
# Root
## Child 1
### Grandchild 1
### Grandchild 2
EOF
)"

"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = LONG_ABOUT)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

impl Args {
    fn run(self) {
        match self.command {
            Command::Vertical(vertical_args) | Command::V(vertical_args) => vertical_args.run(),
            Command::Horizontal(horizontal_args) | Command::H(horizontal_args) => {
                horizontal_args.run()
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print the tree virtually
    Vertical(VerticalArgs),
    // Print the tree virtually (alias for vertical subcommand)
    V(VerticalArgs),
    /// Print the tree horizontally
    Horizontal(HorizontalArgs),
    // Print the tree horizontally (alias for horizontal subcommand)
    H(HorizontalArgs),
}

#[derive(Parser, Debug)]
pub struct HorizontalArgs {
    /// The input filename or content
    #[clap(short, long)]
    input: String,
}

impl HorizontalArgs {
    fn run(&self) {
        // Don't support automatically adding line breaks for horizontal tree
        let root_nodes = parse(&self.input, None);
        horizontal::print_nodes_std(&root_nodes)
    }
}

#[derive(Parser, Debug)]
pub struct VerticalArgs {
    #[clap(short, long, value_enum, default_value = "thin")]
    style: Style,

    /// The input filename or content
    #[clap(short, long)]
    input: String,

    /// The maximum width of each box
    #[clap(short, long)]
    width: Option<usize>,

    /// The horizontal spacing between boxes
    #[clap(long, default_value_t = 2)]
    spacing: usize,
}

impl VerticalArgs {
    fn run(self) {
        let root_nodes = parse(&self.input, self.width);
        for root in root_nodes {
            let result = render(&root, &BoxDrawings::new(self.style), self.spacing);
            println!("{}", result);
        }
    }
}

fn main() {
    let args = Args::parse();
    args.run();
}
