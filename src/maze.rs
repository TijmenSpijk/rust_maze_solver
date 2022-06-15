use image::GrayImage;
use crate::node::*;

pub struct Maze {
    image:      GrayImage,
    node_image: GrayImage,
    path_image: GrayImage,
    width:      u32,
    height:     u32,
    size:       u32,
    nodes:      Vec<Node>
}

impl Maze {
    pub fn new(image: GrayImage) -> Maze {
        Maze { 
            image:      image.clone(),
            node_image: image.clone(),
            path_image: image.clone(),
            width:      image.width(),
            height:     image.height(),
            size:       image.width() * image.height(),
            nodes:      vec![]
        }
    }

    pub fn parse(mut self) {
        self.create_graph();
    }

    fn create_graph(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.image[(x,y)] == image::Luma([0]) {
                    continue;
                }
            }
        }
    }

    fn find_start(&mut self) {

    }

    fn find_end(&mut self) {

    }

    fn write_nodes(&mut self) {

    }

    fn write_solution(&mut self) {

    }
}