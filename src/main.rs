use image::{RgbImage, GrayImage, DynamicImage};
use crate::maze::*;

mod node;
mod maze;

fn main() {
    let image: DynamicImage = image::open("./images/tiny.png").unwrap();
    let color: RgbImage = image.to_rgb8();
    let gray: GrayImage = image.to_luma8();
    let mut maze = Maze::new(color, gray);
    println!("Parsing Maze");
    maze.parse();
    println!("Printing Maze:\n");
    maze.print_maze();
    println!("\nPrinting Nodes:\n");
    maze.print_nodes();
    println!("\nSaving Nodes to Image");
    maze.save_nodes();
}