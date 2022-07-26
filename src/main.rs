extern crate itertools;

use crate::parser::parse;
use crate::tree::drawable::DrawableTreeNode;
use crate::tree::style::BoxDrawings;
use clap::Parser;

mod parser;
mod tree;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The input filename
    #[clap(short, long, value_parser)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let root = parse(args.input);
    let drawable_root = DrawableTreeNode::new(&root);
    let result = drawable_root.render(&BoxDrawings::THIN);
    println!("{}", result);
}
