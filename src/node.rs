use std::rc::Rc;
use std::fmt;

#[derive(PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub enum Node {
    Wall(Wall),
    Path(Path),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Wall(Wall { x, y }) => write!(f, "Wall"),
            Node::Path(Path { x, y, start, end, up, down, left, right }) => write!(f, "Path"),
        }
    }
}

#[derive(Clone)]
pub struct Path {
    x: u32,
    y: u32,
    start: bool,
    end: bool,
    up: Option<Rc<Node>>,
    down: Option<Rc<Node>>,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

impl Path {
    pub fn new(x: u32, y: u32, start: bool, end: bool) -> Path {
        Path {
            x: x,
            y: y,
            start: start,
            end: end,
            up: None,
            down: None,
            left: None,
            right: None
        }
    }
}

#[derive(Clone)]
pub struct Wall {
    x: u32,
    y: u32
}

impl Wall {
    pub fn new(x: u32, y: u32) -> Wall {
        Wall {
            x: x,
            y: y,
        }
    }
}