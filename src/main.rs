extern crate itertools;

use crate::parser::parse;
use crate::tree::drawable::DrawableTreeNode;
use crate::tree::style::BoxDrawings;
use clap::Parser;

mod parser;
mod tree;

const THIN: BoxDrawings = BoxDrawings {
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

    let drawable_root = DrawableTreeNode::new(root.borrow());

    let mut canvas: Vec<Vec<char>> =
        vec![vec![' '; drawable_root.overall_width]; drawable_root.overall_height];

    drawable_root.render(&mut canvas, THIN);
    let result = canvas
        .iter()
        .map(|row| row.iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", result);
}
