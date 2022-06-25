#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Path,
    Node(Node),
}

#[derive(PartialEq, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    id: u32,
    x: u32,
    y: u32,
    start: bool,
    end: bool,
    up: Option<u32>,
    down: Option<u32>,
    left: Option<u32>,
    right: Option<u32>,
}

impl Node {
    pub fn new(id: u32, (x,y): (u32, u32), start: bool, end: bool) -> Node {
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

    pub fn connect(&mut self, dir: Dir, neighbor: u32) {
        match dir {
            Dir::Up => if self.up == None {
                self.up = Some(neighbor);
            },
            Dir::Down => if self.down == None {
                self.down = Some(neighbor);
            },
            Dir::Left => if self.left == None {
                self.left = Some(neighbor);
            },
            Dir::Right => if self.right == None {
                self.right = Some(neighbor);
            },
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_coords(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn get_neighbors(&self) -> [Option<u32>;4] {
        [self.up,self.down,self.left,self.right]
    }

    pub fn _is_start(&self) -> bool {
        self.start
    }

    pub fn _is_end(&self) -> bool {
        self.end
    }
}