#[derive(Clone, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Path
}

#[derive(Clone, PartialEq, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}


#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    id: usize,
    x: u32,
    y: u32,
    start: bool,
    end: bool,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    pub fn new(id: usize, x: u32, y: u32, start: bool, end: bool) -> Node {
        Node {
            id: id,
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

    pub fn connect(&mut self, dir: Dir, neighbor: usize) {
        match dir {
            Dir::Up => if self.up == None {
                self.up = Some(neighbor)
            },
            Dir::Down => if self.down == None {
                self.down = Some(neighbor)
            },
            Dir::Left => if self.left == None {
                self.left = Some(neighbor)
            },
            Dir::Right => if self.right == None {
                self.right = Some(neighbor)
            },
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_coords(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn get_neighbors(&self) -> [Option<usize>;4] {
        [self.up,self.down,self.left,self.right]
    }

    pub fn _is_start(&self) -> bool {
        self.start
    }

    pub fn _is_end(&self) -> bool {
        self.end
    }
}