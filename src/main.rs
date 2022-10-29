extern crate clap;
extern crate itertools;

use crate::parser::parse;

use crate::tree::style::BoxDrawings;
use crate::tree::style::Style;
use crate::tree::vertical::DrawableTreeNode;
use clap::{Parser, Subcommand};
use tree::horizontal;

mod parser;
mod test_utils;
mod tree;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

impl Args {
    fn run(self) {
        match self.command {
            Command::Vertical(vertical_args) => vertical_args.run(),
            Command::Horizontal(horizontal_args) => horizontal_args.run(),
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
    /// The input filename or content
    #[clap(short, long)]
    input: String,
}

impl HorizontalArgs {
    fn run(&self) {
        let root_nodes = parse(&self.input);
        horizontal::print_nodes_std(&root_nodes)
    }
}

#[derive(Parser, Debug)]
pub struct VerticalArgs {
    #[clap(short, long, arg_enum, default_value = "thin")]
    style: Style,

    /// The input filename or content
    #[clap(short, long)]
    input: String,
}

impl VerticalArgs {
    fn run(self) {
        let root_nodes = parse(&self.input);
        for root in root_nodes {
            let drawable_root = DrawableTreeNode::new(&root);
            let result = drawable_root.render(&BoxDrawings::new(self.style));
            println!("{}", result);
        }
    }
}

fn main() {
    let args = Args::parse();
    args.run();
}
