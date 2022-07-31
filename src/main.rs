extern crate itertools;

use crate::parser::parse;
use crate::tree::drawable::DrawableTreeNode;
use crate::tree::style::BoxDrawings;
use crate::tree::style::Style;
use clap::Parser;

mod parser;
mod tree;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The input filename
    #[clap(short, long, value_parser)]
    input: String,

    #[clap(short, long, arg_enum, default_value = "thin")]
    style: Style,
}

fn main() {
    let args = Args::parse();
    let root = parse(args.input);
    let drawable_root = DrawableTreeNode::new(&root);
    let result = drawable_root.render(&BoxDrawings::new(args.style));
    println!("{}", result);
}
