use std::collections::HashSet;
use crate::tiles::*;

pub struct Solution {
    pub algorithm: String,
    pub path: Vec<u32>,
    pub checked: HashSet<Node>,
}

pub fn depth_first_search(nodes: Vec<Node>) -> Solution {
    let mut stack: Vec<(Node, Vec<u32>)> = vec![];
    let mut checked: HashSet<Node> = HashSet::new();
    stack.push((nodes[0], vec![]));
    
    loop {
        match stack.pop() {
            Some((node, path)) => {
                if checked.insert(node) {
                    let mut new_path = path.clone();
                    new_path.push(node.get_id());
                    if node._is_end() {                        
                        return Solution {
                            algorithm: String::from("depth_first_search"),
                            path: new_path,
                            checked: checked
                        };
                    }
                    let neighbors = node.get_neighbors();
                    for neighbor_id in neighbors {
                        match neighbor_id {
                            Some(i) => {
                                let neighbor = nodes[i as usize];
                                stack.push((neighbor, new_path.clone()));
                            },
                            None => ()
                        }
                    }
                }
            },
            None => break
        }
    }
    panic!("No solution found")
}

fn breadth_first_search(nodes: Vec<Node>) {

}

fn dijkstra(nodes: Vec<Node>) {

}

fn astar(nodes: Vec<Node>) {

}