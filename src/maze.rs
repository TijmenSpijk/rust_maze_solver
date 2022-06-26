use std::env;
use image::{RgbImage, GrayImage, DynamicImage};
use crate::tiles::*;
use crate::solve::*;

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
        
        Maze {
            width:  image.width(),
            height: image.height(),
            image:  gray.clone(),
            filename:   String::from(filename),

            node_image: color.clone(),
            solution_image: color.clone(),
            
            maze: maze,
            nodes: vec![],
            node_count: 0,
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
    nodes: Vec<Node>,
    node_count: u32,
}

/* IMPLEMENTATION FOR CREATING A NEW MAZE */
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

        Ok(Maze {
            width:  image.width(),
            height: image.height(),            
            image:  gray.clone(),
            filename:   String::from(filename),

            node_image: color.clone(),
            solution_image: color.clone(),
            
            maze:  vec![vec![Tile::Wall; width]; height],
            nodes: vec![],
            node_count: 0,
        })
    }

    fn get_file_name(path: String) -> String {
        let filename = path.split('/').last().unwrap().split('.').next().unwrap();
        filename.clone().to_owned()
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }
}

/* IMPLEMENTATION FOR FILTERING THE NODES AND CONNECTING THEM */
impl Maze {
    pub fn parse(&mut self) {
        let y = 0;
        for x in 1..self.width-1 {
            if self.image[(x,y)] ==image::Luma([255]) {
                let new_node = Node::new(self.node_count, (x, y), true, false);
                self.maze[x as usize][y as usize] = Tile::Node(new_node);
                self.nodes.push(new_node);
                self.node_count += 1;
            } 
        }

        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                if self.image[(x,y)] == image::Luma([255]) {
                    if !self.is_corridor(x, y) {          
                        let new_node = Node::new(self.node_count, (x, y), false, false);
                        self.maze[x as usize][y as usize] = Tile::Node(new_node);    
                        self.nodes.push(new_node);
                        self.connect_nodes_left_right(self.node_count, (x,y));                
                        self.connect_nodes_up_down(self.node_count, (x,y));
                        self.node_count += 1; 
                    } else {
                        self.maze[x as usize][y as usize] = Tile::Path;
                    }
                }
            }
        }

        let y = self.height-1;
        for x in 1..self.width-1 {
            if self.image[(x,y)] ==image::Luma([255]) {
                let new_node = Node::new(self.node_count, (x, y), false, true);
                self.maze[x as usize][y as usize] = Tile::Node(new_node);
                self.nodes.push(new_node);
                self.connect_nodes_left_right(self.node_count, (x,y));                
                self.connect_nodes_up_down(self.node_count, (x,y));
                self.node_count += 1;                
            } 
        }
    }

    fn connect_nodes_left_right(&mut self, new_id: u32, (x,y): (u32, u32)) {
        for i in (0..x).rev() {
            println!("{:?}", &self.maze[i as usize][y as usize]);
            match &self.maze[i as usize][y as usize] {
                Tile::Wall => break,
                Tile::Path => continue,
                Tile::Node(node) => {
                    let old_id = node.get_id();
                    self.nodes[old_id as usize].connect(Dir::Right, new_id);
                    self.nodes[new_id as usize].connect(Dir::Left, old_id);
                    break;
                }
            }
        }
    }

    fn connect_nodes_up_down(&mut self, new_id: u32, (x,y): (u32, u32)) {
        for i in (0..y).rev() {
            println!("{:?}", &self.maze[x as usize][i as usize]);
            match &self.maze[x as usize][i as usize] {
                Tile::Wall => break,
                Tile::Path => continue,
                Tile::Node(node) => {
                    let old_id = node.get_id();
                    self.nodes[old_id as usize].connect(Dir::Down, new_id);
                    self.nodes[new_id as usize].connect(Dir::Up, old_id);
                    break;
                }
            }
        }
    }

    fn is_corridor(&mut self, x: u32, y: u32) -> bool {
        let x     = x;
        let y     = y;
        let left  = x-1;
        let right = x+1;
        let up    = y-1;
        let down  = y+1;

        let horizontal = self.image[(left, y)] == image::Luma([0]) && self.image[(right, y)] == image::Luma([0]) && self.image[(x, up)] != image::Luma([0]) && self.image[(x, down)] != image::Luma([0]);
        let vertical = self.image[(left, y)] != image::Luma([0]) && self.image[(right, y)] != image::Luma([0]) && self.image[(x, up)] == image::Luma([0]) && self.image[(x, down)] == image::Luma([0]);

        horizontal || vertical
    }
}

/* IMPLEMENTATION FOR WRITING TO THE IMAGES AND SAVING THEM */
impl Maze {
    pub fn save_nodes(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match &self.maze[x as usize][y as usize] {
                    Tile::Node(node) => {
                        if node._is_start() {
                            self.node_image[(x,y)] = image::Rgb([0,0,255])
                        } else if node._is_end() {
                            self.node_image[(x,y)] = image::Rgb([0,255,0])
                        } else {
                            self.node_image[(x,y)] = image::Rgb([255,0,0])
                        }
                    },
                    _ => ()
                }
            }
        }

        match self.node_image.save("images/processed/".to_owned() + &self.filename + "_nodes.png") {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err)
        }
    }

    pub fn save_solution(&mut self, solution: Solution) {
        for node in solution.checked {
            let (x,y) = node.get_coords();
            self.solution_image[(x,y)] = image::Rgb([0,0,255])
        }

        match self.solution_image.save("images/processed/".to_owned() + &self.filename + "_solution.png") {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err)
        }
    }
}