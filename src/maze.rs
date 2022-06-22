use std::env;
use image::{RgbImage, GrayImage, DynamicImage};
use crate::tiles::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_maze() -> Maze{
        let filename = "tiny";

        let image: DynamicImage = image::open("images/".to_owned() + &filename + ".png").unwrap();
        let color: RgbImage = image.to_rgb8();
        let gray: GrayImage = image.to_luma8();

        let width: usize = image.width() as usize;
        let height: usize = image.height() as usize;
        let maze: Vec<Vec<Tile>> = vec![vec![Tile::Wall; width]; height];
        let nodes: Vec<Node> = vec![];
        
        Maze {
            width:  image.width(),
            height: image.height(),            
            image:  gray.clone(),
            filename:   String::from(filename),

            node_image: color.clone(),
            solution_image: color.clone(),
            
            maze:  maze,
            nodes: nodes,
        }
    }

    #[test]
    fn has_start_and_end() {
        let mut maze = get_maze();
        maze.parse();
        let maze_start  = true;
        let maze_end    = true;
        let nodes_start = true;
        let nodes_end   = true;
        assert!(maze_start && maze_end && nodes_start && nodes_end)
    }
}

#[derive(Debug)]
pub struct Maze {
    width:  u32,
    height: u32,

    image:  GrayImage,
    filename:   String,

    node_image: RgbImage,
    solution_image: RgbImage,

    maze:  Vec<Vec<Tile>>,
    nodes: Vec<Node>
}

impl Maze {
    pub fn new(mut args: env::Args) -> Result<Maze, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => Maze::get_file_name(arg),
            None => return Err("No path to input file given")
        };
        
        let image: DynamicImage = image::open("images/".to_owned() + &filename + ".png").unwrap();
        let color: RgbImage = image.to_rgb8();
        let gray: GrayImage = image.to_luma8();

        let width: usize = image.width() as usize;
        let height: usize = image.height() as usize;
        let maze: Vec<Vec<Tile>> = vec![vec![Tile::Wall; width]; height];
        let nodes: Vec<Node> = vec![];

        Ok(Maze {
            width:  image.width(),
            height: image.height(),            
            image:  gray.clone(),
            filename:   String::from(filename),

            node_image: color.clone(),
            solution_image: color.clone(),
            
            maze:  maze,
            nodes: nodes,
        })
    }

    fn get_file_name(path: String) -> String {
        let filename = path.split('/').last().unwrap().split('.').next().unwrap();
        println!("{}", &filename);
        filename.clone().to_owned()
    }

    pub fn parse(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.image[(x,y)] {
                    image::Luma([255]) => self.maze[x as usize][y as usize] = Tile::Path,
                    _ => ()
                }
            }
        }        
        self.filter_nodes();
    }

    fn filter_nodes(&mut self) {
        self.filter_start();
        for y in 1..self.height-1 {
            self.filter_row(y);
        }
        self.filter_end();
    }

    fn filter_start(&mut self) {
        let y = 0;
        for x in 1..self.width-1 {
            match self.maze[x as usize][y as usize] {
                Tile::Path => {
                    println!("Found Node");
                    self.nodes.push(Node::new(x, y, true, false));
                },
                Tile::Wall => continue                    
            }                
        }
    }

    fn filter_end(&mut self) {
        let y = (self.height - 1) as usize;
        for x in 1..self.width-1 {
            match self.maze[x as usize][y] {
                Tile::Path => {
                    self.nodes.push(Node::new(x, y as u32, false, true));                    
                    break;
                },
                Tile::Wall => continue
            }
        }
    }

    fn filter_row(&mut self, y: u32) {
        for x in 1..self.width-1 {
            match self.maze[x as usize][y as usize] {
                Tile::Path =>
                    if !self.is_corridor(x, y) {
                        self.nodes.push(Node::new(x, y, false, false));
                    },
                Tile::Wall => continue                    
            }                
        }
    }

    fn check_connect(&mut self, x:u32, y: u32) {

    }

    fn is_corridor(&mut self, x: u32, y: u32) -> bool {
        let x     = x as usize;
        let y     = y as usize;
        let left  = x-1;
        let right = x+1;
        let up    = y-1;
        let down  = y+1;

        let horizontal = self.maze[left][y] == Tile::Wall && self.maze[right][y] == Tile::Wall && self.maze[x][up] != Tile::Wall && self.maze[x][down] != Tile::Wall;
        let vertical = self.maze[left][y] != Tile::Wall && self.maze[right][y] != Tile::Wall && self.maze[x][up] == Tile::Wall && self.maze[x][down] == Tile::Wall;

        horizontal || vertical
    }

    pub fn save_nodes(&mut self) {
        for node in &self.nodes {
            let (x,y) = node.get_coords();
            self.node_image[(x,y)] = image::Rgb([255,0,0]);
        }

        match self.node_image.save("images/processed/".to_owned() + &self.filename + "_nodes.png") {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err)
        }
    }

    pub fn save_solution(&mut self) {

    }
}