use std::env;
use image::{RgbImage, GrayImage, DynamicImage};
use crate::node::*;

pub struct Maze {
    width:  u32,
    height: u32,

    image:  GrayImage,
    filename:   String,

    node_image: RgbImage,
    solution_image: RgbImage,

    maze:  Vec<Vec<Option<Node>>>,
    nodes: Vec<Vec<Option<Node>>>
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
        let maze: Vec<Vec<Option<Node>>> = vec![vec![None; width]; height];
        let nodes: Vec<Vec<Option<Node>>> = vec![vec![None; width]; height];

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

    pub fn print_maze(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let node = &self.maze[x as usize][y as usize];
                match node {
                    Some(_) => print!(" "),
                    None => print!("X")
                }
            }
            println!();
        }
    }

    pub fn print_nodes(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.nodes[x as usize][y as usize] {
                    Some(_) => print!("O"),
                    None => print!(" ")
                }
            }
            println!();
        }
    }

    pub fn parse(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.image[(x,y)] == image::Luma([255]) {
                    self.maze[x as usize][y as usize] = Some(Node::new(x, y, x==0, y==self.height-1));
                }
            }
        }
        self.filter_nodes();
    }

    fn get_file_name(path: String) -> String {
        let filename = path.split('/').last().unwrap().split('.').next().unwrap();
        println!("{}", &filename);
        filename.clone().to_owned()
    }

    fn filter_nodes(&mut self) {
        self.filter_start();
        for y in 1..self.height-1 {
            self.filter_row(y);
        }
        self.filter_end();
    }

    fn filter_start(&mut self) {
        for x in 1..self.width-1 {
            match self.maze[x as usize][0 as usize] {
                Some(_) => self.nodes[x as usize][0 as usize] = Some(Node::new(x, 0, x==0, 0==self.height-1)),                    
                None => continue                    
            }                
        }
    }

    fn filter_end(&mut self) {
        let y = (self.height - 1) as usize;
        for x in 1..self.width-1 {
            match self.maze[x as usize][y] {
                Some(_) => self.nodes[x as usize][y] = Some(Node::new(x, y as u32, x==0, 0==self.height-1)),                    
                None => continue                    
            }                
        }
    }

    fn filter_row(&mut self, y: u32) {
        for x in 1..self.width-1 {
            match self.maze[x as usize][y as usize] {
                Some(_) =>
                    if !self.is_corridor(x, y) {
                        self.nodes[x as usize][y as usize] = Some(Node::new(x, y, x==0, y==self.height-1));
                    },
                None => continue                    
            }                
        }
    }

    fn is_corridor(&mut self, x: u32, y: u32) -> bool {
        let x     = x as usize;
        let y     = y as usize;
        let left  = x-1;
        let right = x+1;
        let up    = y-1;
        let down  = y+1;

        let horizontal = self.maze[left][y] == None && self.maze[right][y] == None && self.maze[x][up] != None && self.maze[x][down] != None;
        let vertical = self.maze[left][y] != None && self.maze[right][y] != None && self.maze[x][up] == None && self.maze[x][down] == None;

        horizontal || vertical
    }

    pub fn save_nodes(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.nodes[x as usize][y as usize] {
                    Some(node) => {
                        let (px,py) = node.get_coords();
                        self.node_image[(px, py)] = image::Rgb([255,0,0]);
                    },
                    None => continue
                }
            }
        }
        match self.node_image.save("images/processed/".to_owned() + &self.filename + "_nodes.png") {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err)
        }
    }

    pub fn save_solution(&mut self) {

    }
}