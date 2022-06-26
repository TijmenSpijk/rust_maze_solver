use std::{env, process};

use crate::solve::*;
use crate::maze::*;

mod tiles;
mod maze;
mod solve;

fn main() {
    println!("Reading Input");
    println!("Creating Maze Object");    
    let mut maze = setup_maze();
    println!("Parsing Maze");
    maze.parse();
    println!("Saving Nodes to Image");
    maze.save_nodes();
    println!("Finding Solution");
    for node in maze.get_nodes() {
        println!("{}, {:?}", node.get_id(), node.get_neighbors())
    }
    // let solution = depth_first_search(maze.get_nodes().to_vec());
    // maze.save_solution(solution);
}

fn setup_maze() -> Maze {
    Maze::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    })
}