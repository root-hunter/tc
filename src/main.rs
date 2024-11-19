use std::collections::HashMap;
use priority_queue::{self, PriorityQueue};
use std::cmp::Reverse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pub is_root: bool,
    pub freq: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

fn build_btree(node: Node, dict: &mut HashMap<char, Vec<u8>>, acc: Vec<u8>, depth: usize) {
    if node.left == None && node.right == None {
        //let key = format!("{}{}", acc, bit);

        dict.insert(node.symbol.unwrap(), acc.clone());
    } else {
        let left = node.left.unwrap();
        let right = node.right.unwrap();

        if !node.is_root && depth > 0 {
            let mut acc_left = acc.clone();
            acc_left.push(0);

            let mut acc_rigth = acc.clone();
            acc_rigth.push(1);

            build_btree(*left, dict, acc_left, depth + 1);
            build_btree(*right, dict, acc_rigth, depth + 1);
        } else {
            let acc_left = Vec::from([0]);
            let acc_rigth = Vec::from([1]);

            build_btree(*left, dict, acc_left, 1);
            build_btree(*right, dict, acc_rigth, 1);
        }
    }
}

fn main() {
    let input_string = "Ciao a tutti mi chiamo Antonio e questa è la prima prova di compressione usando il metodo di Huffman";

    let chars = input_string.chars();

    let mut distribution = HashMap::<char, usize>::new();

    for char in chars.clone() {
        if !distribution.contains_key(&char) {
            distribution.insert(char, 1);
        } else {
            let count = distribution.get_mut(&char).unwrap();

            *count += 1;
        }
    }
    
    assert!(distribution.values().sum::<usize>() == chars.clone().count());
    println!("DIST: {:?}", distribution);

    let mut pq = PriorityQueue::new();

    for (k, v) in distribution {
        pq.push(Node{
            freq: v,
            symbol: Some(k),
            left: None,
            right: None,
            is_root: false
        }, Reverse(v));
    }
    //println!("PQ: {:?}", pq);

    while pq.len() > 1 {
        let (left, _) = pq.pop().unwrap();
        let (right, _) = pq.pop().unwrap();
        //println!("LEFT: {:?}", left);
        //println!("RIGHT: {:?}", right);

        let freq = left.freq + right.freq;
        let node = Node{
            freq,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            is_root: if pq.len() == 1 {
                true
            } else {
                false
            }
        };

        pq.push(node, Reverse(freq));
    }

    //println!("{:#?}", pq);

    let (root, _) = pq.pop().unwrap();

    let mut conversion_dict = HashMap::<char, Vec<u8>>::new();
    let acc = Vec::new();

    build_btree(root, &mut conversion_dict, acc, 0);

    println!("CONVERSION HASH: {:?}", conversion_dict);

    let mut compressed: Vec<u8> = Vec::new();

    for c in chars.clone() {
        //println!("{} -> {:?}", c, conversion_dict.get(&c.clone()));
        let comp = conversion_dict.get(&c.clone()).unwrap().clone();

        for d in comp {
            compressed.push(d);
        }
    }

    println!("COMPRESSED: {:?}", compressed);
    println!("ORIGINAL LENGTH: {}", chars.count());
    println!("COMPRESSED LENGTH: {}", compressed.len() / 8);


    let chunks = compressed.chunks(8);

    let mut output: Vec<u8> = Vec::new();

    for chunk in chunks {
        let mut b: u8 = 0x00;

        let mut position: usize = 0;
        for bit in chunk {
            if *bit == 1 {
                b = b | 1 << 7 - position;
            } else {
                b = b | 0 << 7 - position;
            }
            
            position += 1;
        }
        output.push(b);
        //println!("COMPRESSED CHUNK ({:08b}): {:?}", b, chunk);
    }

    println!("INPUT: {:?}", input_string.as_bytes());
    println!("OUTPUT: {:?}", output);
}