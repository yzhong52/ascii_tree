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
    /// The input filename
    #[clap(short, long, value_parser)]
    input: String,
}

impl HorizontalArgs {
    fn run(&self) {
        let root = parse(&self.input);
        println!(".");
        horizontal::print_nodes(&vec![root], "")
    }
}

#[derive(Parser, Debug)]
pub struct VerticalArgs {
    /// The input filename
    #[clap(short, long, value_parser)]
    input: String,

    #[clap(short, long, arg_enum, default_value = "thin")]
    style: Style,

    #[clap(short, long)]
    top_connection: Option<char>,

    #[clap(short, long)]
    bottom_connection: Option<char>,
}

impl VerticalArgs {
    fn run(self) {
        let root = parse(&self.input);
        let drawable_root = DrawableTreeNode::new(&root);
        let result = drawable_root.render(
            &BoxDrawings::new(self.style),
            self.top_connection,
            self.bottom_connection,
        );
        println!("{}", result);
    }
}

fn main() {
    let args = Args::parse();
    args.run();
}
