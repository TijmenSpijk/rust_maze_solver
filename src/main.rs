use std::{env, process};

use crate::maze::*;

mod tiles;
mod maze;

fn main() {
    let mut maze = setup_maze();

    println!("Parsing Maze");
    maze.parse();
    println!("Saving Nodes to Image");
    maze.save_nodes();
}

fn setup_maze() -> Maze {
    Maze::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    })
}