use image::GrayImage;
use crate::node::*;

pub struct Maze {
    image:      GrayImage,
    node_image: GrayImage,
    path_image: GrayImage,
    width:      u32,
    height:     u32,
    nodes:  Vec<Vec<Option<Node>>>
}

impl Maze {
    pub fn new(image: GrayImage) -> Maze {
        let width: usize = image.width() as usize;
        let height: usize = image.height() as usize;
        let nodes: Vec<Vec<Option<Node>>> = vec![vec![None; width]; height];

        Maze { 
            image:      image.clone(),
            node_image: image.clone(),
            path_image: image.clone(),
            width:      image.width(),
            height:     image.height(),
            nodes:      nodes,
        }
    }

    pub fn print_nodes(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let node = &self.nodes[x as usize][y as usize];
                match node {
                    Some(_) => print!(" "),
                    None => print!("X")
                }
            }
            println!();
        }
    }

    pub fn parse(&mut self) {
        self.parse_row(0, true, false);
        for y in 1..self.height {
            self.parse_row(y, false, false)
        }
        self.parse_row(self.height-1, false, true);
    }

    fn parse_row(&mut self, y: u32, start: bool, end: bool) {
        for x in 0..self.width {
            self.parse_unit(x, y, start, end)
        }
    }

    fn parse_unit(&mut self, x: u32, y: u32, start: bool, end: bool) {
        if self.image[(x,y)] == image::Luma([255]) {
            self.nodes[x as usize][y as usize] = Some(Node::new(x, y, start, end));
        }
    }
}