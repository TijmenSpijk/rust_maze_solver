use std::collections::{HashSet, VecDeque};
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
                    for neighbor_id in node.get_neighbors() {
                        match neighbor_id {
                            Some(i) => {
                                stack.push((nodes[i as usize], new_path.clone()));
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

pub fn breadth_first_search(nodes: Vec<Node>) -> Solution {
    let mut stack: VecDeque<(Node, Vec<u32>)> = VecDeque::new();
    let mut checked: HashSet<Node> = HashSet::new();
    
    checked.insert(nodes[0]);
    stack.push_front((nodes[0], vec![]));

    loop {
        match stack.pop_front() {
            Some((node, path)) => {
                let mut new_path = path.clone();
                new_path.push(node.get_id());
                if node._is_end() {
                    return Solution {
                        algorithm: String::from("breadth_first_search"),
                        path: new_path,
                        checked: checked
                    }
                }
                for neighbor_id in node.get_neighbors() {
                    match neighbor_id {
                        Some(id) => {
                            let neighbor = nodes[id as usize];
                            if checked.insert(neighbor) {
                                stack.push_back((neighbor, new_path.clone()));
                            }
                        },
                        None => ()
                    }                    
                }
            },
            None => break,
        }
    }
    panic!("No solution found")
}

fn dijkstra(nodes: Vec<Node>) {

}

fn astar(nodes: Vec<Node>) {

}