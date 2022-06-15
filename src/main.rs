use image::{GrayImage, DynamicImage};
use crate::maze::*;

mod node;
mod maze;

fn main() {
    let image: DynamicImage = image::open("./images/tiny.png").unwrap();
    let image: GrayImage = image.to_luma8();
    let maze = Maze::new(image);
    maze.parse();
}

