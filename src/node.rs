use std::rc::Rc;

#[derive(Clone)]
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
    pub fn new(x: u32, y: u32, start: bool, end: bool) -> Node {
        Node { 
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