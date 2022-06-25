use std::collections::HashSet;

use crate::maze::*;
use crate::tiles::*;

pub struct Solution {
    pub path: Vec<u32>,
    pub checked: HashSet<Node>,
}

pub fn depth_first_search(nodes: Vec<Node>) -> Solution {
    let mut stack: Vec<Node> = vec![];
    let mut checked: HashSet<Node> = HashSet::new();
    stack.push(nodes[0]);
    
    loop {
        match stack.pop() {
            Some(node) => {
                if checked.insert(node) {
                    let neighbors = node.get_neighbors();
                    for neighbor_id in neighbors {
                        match neighbor_id {
                            Some(i) => {
                                let neighbor = nodes[i as usize];
                                if neighbor._is_end() {
                                    break;
                                }
                                stack.push(neighbor);
                            },
                            None => ()
                        }
                    }
                }
            },
            None => break
        }
    }
    Solution { path: vec![], checked: checked }
}

fn breadth_first_search(nodes: Vec<Node>) {

}

fn dijkstra(nodes: Vec<Node>) {

}

fn astar(nodes: Vec<Node>) {

}