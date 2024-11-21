use priority_queue::{self, PriorityQueue};
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
struct Node {
    pub is_root: bool,
    pub freq: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

fn build_btree(node: Node, dict: &mut HashMap<char, Vec<u8>>, acc: Vec<u8>, depth: usize) {
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

            build_btree(*left, dict, acc_left, 0);
            build_btree(*right, dict, acc_rigth, 1);
        }

    }
}

fn find_symbol(node: Node, sequence: Vec<u8>, depth: usize) -> Option<char> {
    println!("SEQ: {:?}", sequence);
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

fn main() {
    //let input_string = std::fs::read("/home/roothunter/Dev/tc/tests/inputs/input_5.txt").unwrap();
    //let input_string = String::from_utf8(input_string).unwrap();
    let input_string = "AAAABBBBCCCCAAFAA";

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
        pq.push(
            Node {
                freq: v,
                symbol: Some(k),
                left: None,
                right: None,
                is_root: false,
            },
            Reverse(v),
        );
    }
    //println!("PQ: {:?}", pq);

    while pq.len() > 1 {
        let (left, _) = pq.pop().unwrap();
        let (right, _) = pq.pop().unwrap();
        //println!("LEFT: {:?}", left);
        //println!("RIGHT: {:?}", right);

        let freq = left.freq + right.freq;
        let node = Node {
            freq,
            symbol: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            is_root: false,
        };

        pq.push(node, Reverse(freq));
    }

    //println!("{:#?}", pq);

    let (mut root, _) = pq.pop().unwrap();
    root.is_root = true;
    println!("{:?}", root);

    let mut conversion_dict = HashMap::<char, Vec<u8>>::new();
    let acc = Vec::new();

    build_btree(root.clone(), &mut conversion_dict, acc, 0);

    println!("CONVERSION HASH: {:?}", conversion_dict);

    let mut compressed: Vec<u8> = Vec::new();

    for c in chars.clone() {
        println!("{} -> {:?}", c, conversion_dict.get(&c.clone()));
        let comp = conversion_dict.get(&c.clone()).unwrap().clone();

        for d in comp {
            compressed.push(d);
        }
    }

    let mut decompress_map = HashMap::<Vec<u8>, char>::new();


    for (k, v) in conversion_dict {
        decompress_map.insert(v, k);
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

    let mut decompressed = String::new();
    let mut buffer = Vec::<u8>::new();

    let mut output_string = String::new();

    for cc in compressed {
        println!("C: {}", cc);
        buffer.push(cc);
        if buffer.len() > 0 {
            if let Some(char) = decompress_map.get(&buffer) {
                println!("{:?} -> {}", buffer, *char);
                output_string.push(*char);
                buffer.clear();
            }
        }
    }


    println!("OUTPUT STRING: {}", output_string);

    println!("INPUT: {:?}", input_string.as_bytes());
    println!("OUTPUT: {:?}", output);
}
