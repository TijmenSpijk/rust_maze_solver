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
        
        Maze {
            width:  image.width(),
            height: image.height(),
            image:  gray.clone(),
            filename:   String::from(filename),

            node_image: color.clone(),
            solution_image: color.clone(),
            
            maze:  maze,
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
            node_count: 0,
        })
    }

    fn get_file_name(path: String) -> String {
        let filename = path.split('/').last().unwrap().split('.').next().unwrap();
        filename.clone().to_owned()
    }
}

/* IMPLEMENTATION FOR FILTERING THE NODES AND CONNECTING THEM */
impl Maze {
    pub fn parse(&mut self) {
        let y = 0;
        for x in 1..self.width-1 {
            if self.image[(x,y)] ==image::Luma([255]) {
                self.maze[x as usize][y as usize] = Tile::Node(Node::new(self.node_count, (x, y), true, false));
                self.node_count += 1;
            } 
        }

        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                if self.image[(x,y)] == image::Luma([255]) {
                    if !self.is_corridor(x, y) {          
                        let mut new_node = Node::new(self.node_count, (x, y), false, false);
                        self.node_count += 1;                        
                        self.check_left(&mut new_node, (x,y));
                        self.check_up(&mut new_node, (x,y));
                        self.maze[x as usize][y as usize] = Tile::Node(new_node);
                    } else {
                        self.maze[x as usize][y as usize] = Tile::Path;
                    }
                }
            }
        }

        let y = self.height-1;
        for x in 1..self.width-1 {
            if self.image[(x,y)] ==image::Luma([255]) {
                let mut new_node = Node::new(self.node_count, (x, y), false, true);
                self.node_count += 1;                
                self.check_left(&mut new_node, (x,y));
                self.check_up(&mut new_node, (x,y));
                self.maze[x as usize][y as usize] = Tile::Node(new_node);
            } 
        }
    }

    fn check_left(&mut self, new_node: &mut Node, (x,y): (u32, u32)) {
        for i in (0..x).rev() {
            match &self.maze[i as usize][y as usize] {
                Tile::Wall => return,
                Tile::Path => continue,
                Tile::Node(mut node) => {
                    new_node.connect(Dir::Left, node.get_id());
                    node.connect(Dir::Right, new_node.get_id());
                }
            }
        }
    }

    fn check_up(&mut self, new_node: &mut Node, (x,y): (u32, u32)) {
        for i in (0..y).rev() {
            match &self.maze[x as usize][i as usize] {
                Tile::Wall => return,
                Tile::Path => continue,
                Tile::Node(mut node) => {
                    new_node.connect(Dir::Up, node.get_id());
                    node.connect(Dir::Down, new_node.get_id());
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
                    Tile::Node(_) => self.node_image[(x,y)] = image::Rgb([255,0,0]),
                    _ => ()
                }
            }
        }

        match self.node_image.save("images/processed/".to_owned() + &self.filename + "_nodes.png") {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err)
        }
    }
}