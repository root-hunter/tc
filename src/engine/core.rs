use std::collections::HashMap;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Node {
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

    #[serde(skip)]
    pub freq: usize,
}


#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Data {
    pub length: usize,
    pub dict: Node,
    pub data: Vec<u8>,
}


pub fn build_dict(node: Node, dict: &mut HashMap<char, Vec<u8>>, acc: Vec<u8>, depth: usize, is_root: bool) {
    // println!("##########################################");
    // if depth == 0 {
    //     println!("ROOT: {:?}\nDEPTH: {}\nACC: {:?}\nMAP: {:?}",  node, depth, acc, dict);
    // } else {
    //     println!("NODE: {:?}\nDEPTH: {}\nACC: {:?}\nMAP: {:?}", node, depth, acc, dict);
    // }
    
    if node.left == None && node.right == None && node.symbol.is_some() {
        dict.insert(node.symbol.unwrap(), acc.clone());
    } else {
        let left = node.left.unwrap();
        let right = node.right.unwrap();

        if !is_root && depth > 0 {
            let mut acc_left = Vec::from(acc.clone());
            acc_left.push(0);

            let mut acc_rigth = Vec::from(acc.clone());
            acc_rigth.push(1);

            build_dict(*left, dict, acc_left, depth + 1, false);
            build_dict(*right, dict, acc_rigth, depth + 1, false);
        } else {
            let acc_left = Vec::from([0]);
            let acc_rigth = Vec::from([1]);

            build_dict(*left, dict, acc_left,1, false);
            build_dict(*right, dict, acc_rigth, 1, false);
        }

    }
}

pub fn find_symbol(dict: Node, sequence: Vec<u8>, depth: usize) -> Option<char> {
    //println!("SEQ: {:?}", sequence);
    // println!("ACC: {:?}", sequence);
    // println!("DEPTH: {:?}", depth);
    // println!("{:?}", node);

    if depth == 0 {
        if dict.symbol.is_some() {
            return Some(dict.symbol.unwrap());
        } else {
            return None;
        }
    } else {
        if dict.left != None && dict.right != None {
            let left = dict.left.unwrap();
            let right = dict.right.unwrap();

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