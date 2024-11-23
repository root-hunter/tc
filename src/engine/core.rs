use priority_queue::{self, PriorityQueue};
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Node {
    pub is_root: bool,
    pub freq: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub fn build_btree(node: Node, dict: &mut HashMap<char, Vec<u8>>, acc: Vec<u8>, depth: usize) {
    println!("##########################################");
    if depth == 0 {
        println!("ROOT: {:?}\nDEPTH: {}\nACC: {:?}\nMAP: {:?}",  node, depth, acc, dict);
    } else {
        println!("NODE: {:?}\nDEPTH: {}\nACC: {:?}\nMAP: {:?}", node, depth, acc, dict);
    }
    
    if node.left == None && node.right == None && node.symbol.is_some() {
        dict.insert(node.symbol.unwrap(), acc.clone());
    } else {
        let left = node.left.unwrap();
        let right = node.right.unwrap();

        if !node.is_root && depth > 0 {
            let mut acc_left = Vec::from(acc.clone());
            acc_left.push(0);

            let mut acc_rigth = Vec::from(acc.clone());
            acc_rigth.push(1);

            build_btree(*left, dict, acc_left, depth + 1);
            build_btree(*right, dict, acc_rigth, depth + 1);
        } else {
            let acc_left = Vec::from([0]);
            let acc_rigth = Vec::from([1]);

            build_btree(*left, dict, acc_left,1);
            build_btree(*right, dict, acc_rigth, 1);
        }

    }
}

pub fn find_symbol(node: Node, sequence: Vec<u8>, depth: usize) -> Option<char> {
    //println!("SEQ: {:?}", sequence);
    // println!("ACC: {:?}", sequence);
    // println!("DEPTH: {:?}", depth);
    // println!("{:?}", node);

    if depth == 0 {
        if node.symbol.is_some() {
            return Some(node.symbol.unwrap());
        } else {
            return None;
        }
    } else {
        if node.left != None && node.right != None {
            let left = node.left.unwrap();
            let right = node.right.unwrap();

            let first = sequence.get(sequence.len() - depth).unwrap();
            //println!("FIRST {}", first);

            if *first == 0 {
                //println!("LEFT");
                return find_symbol(*left, sequence, depth - 1);
            } else {
                //println!("RIGHT");
                return find_symbol(*right, sequence, depth - 1);
            }
        } else {
            return None;
        }
    }
}
