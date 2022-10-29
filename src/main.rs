extern crate clap;
extern crate itertools;

use crate::parser::parse;

use crate::tree::style::BoxDrawings;
use crate::tree::style::Style;
use crate::tree::vertical::DrawableTreeNode;
use clap::{Parser, Subcommand};
use tree::horizontal;

mod parser;
mod tree;
mod test_utils;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The input filename or content
    #[clap()]
    input: String,

    #[clap(subcommand)]
    command: Command,
}

impl Args {
    fn run(self) {
        match self.command {
            Command::Vertical(vertical_args) => vertical_args.run(self.input),
            Command::Horizontal(horizontal_args) => horizontal_args.run(self.input),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print the tree virtually
    Vertical(VerticalArgs),
    /// Print the tree horizontally
    Horizontal(HorizontalArgs),
}

#[derive(Parser, Debug)]
pub struct HorizontalArgs {
}

impl HorizontalArgs {
    fn run(&self, input: String) {
        let root_nodes = parse(&input);
        println!(".");
        horizontal::print_nodes_std(&root_nodes)
    }
}

#[derive(Parser, Debug)]
pub struct VerticalArgs {
    #[clap(short, long, arg_enum, default_value = "thin")]
    style: Style,

    #[clap(short, long)]
    top_connection: Option<char>,

    #[clap(short, long)]
    bottom_connection: Option<char>,
}

impl VerticalArgs {
    fn run(self, input: String) {
        let root_nodes = parse(&input);
        for root in root_nodes {
            let drawable_root = DrawableTreeNode::new(&root);
            let result = drawable_root.render(
                &BoxDrawings::new(self.style),
                self.top_connection,
                self.bottom_connection,
            );
            println!("{}", result);
        }
    }
}

fn main() {
    let args = Args::parse();
    args.run();
}
