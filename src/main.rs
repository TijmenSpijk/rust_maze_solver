use image::{GrayImage, DynamicImage};
use crate::maze::*;

mod node;
mod maze;

fn main() {
    let image: DynamicImage = image::open("./images/tiny.png").unwrap();
    let image: GrayImage = image.to_luma8();
    let mut maze = Maze::new(image);
    println!("Parsing Maze");
    maze.parse();
    println!("Printing Nodes");
    maze.print_nodes();
}