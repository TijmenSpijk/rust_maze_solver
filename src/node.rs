use std::rc::Rc;

#[derive(PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Node {
    x: u32,
    y: u32,
    start: bool,
    end: bool,
    up: Option<Rc<Node>>,
    down: Option<Rc<Node>>,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

impl Node {
    pub fn new(x: u32, y: u32) -> Node {
        Node {
            x: x,
            y: y,
            start: false,
            end: false,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }

    pub fn connect(mut self, node: Rc<Node>, dir: Dir) {
        if dir == Dir::Up {
            self.up = Some(node);
        } else if dir == Dir::Down {
            self.down = Some(node);
        } else if dir == Dir::Left {
            self.left = Some(node);
        } else if dir == Dir::Right {
            self.right = Some(node);
        } else {
            panic!("Incorrect dir")
        }
    }
}