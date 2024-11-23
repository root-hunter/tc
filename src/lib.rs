use std::{cmp::Reverse, collections::HashMap};

use engine::core::{build_btree, Node};
use priority_queue::PriorityQueue;

pub mod engine {
    pub mod core;
}

pub fn compress(buf: &[u8]) -> Vec<u8> {
    let input_string: String = String::from_utf8(buf.to_vec()).unwrap();
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

    return compressed;
}

pub fn compress_file(file_path: &str, output_path: &str) -> Result<(), ()>{
    if let Ok(file_data) = std::fs::read(file_path){
        let data = compress(file_data.as_slice());

        if let Ok(_) = std::fs::write(output_path, data) {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    return Vec::new();
}

pub fn decompress_file(file_path: &str, output_path: &str) -> Result<(), ()>{
    if let Ok(file_data) = std::fs::read(file_path) {
        let data = decompress(file_data.as_slice());
        if let Ok(_) = std::fs::write(output_path, data) {
            Ok(())
        } else {
            Err(())
        }      
    } else {
        Err(())
    }
}