use std::{cmp::Reverse, collections::HashMap};

use engine::core::{build_dict, find_symbol, Data, Node};
use priority_queue::PriorityQueue;

pub mod engine {
    pub mod core;
}

pub fn compress(input_string: String) -> Data {
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
    //println!("DIST: {:?}", distribution);

    let mut pq = PriorityQueue::new();

    for (k, v) in distribution {
        pq.push(
            Node {
                freq: v,
                symbol: Some(k),
                left: None,
                right: None,
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
        };

        pq.push(node, Reverse(freq));
    }

    //println!("{:#?}", pq);

    let (root, _) = pq.pop().unwrap();
    //println!("{:?}", root);

    let mut conversion_dict = HashMap::<char, Vec<u8>>::new();
    let acc = Vec::new();

    build_dict(root.clone(), &mut conversion_dict, acc, 0, true);
    let mut compressed: Vec<u8> = Vec::new();

    for c in chars.clone() {
        //println!("{} -> {:?}", c, conversion_dict.get(&c.clone()));
        let comp = conversion_dict.get(&c.clone()).unwrap().clone();

        for d in comp {
            compressed.push(d);
        }
    }
    let compressed_len = compressed.len();

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

    return Data{
        length: compressed_len,
        dict: root,
        data: output
    };
}

pub fn compress_file(file_path: &str, output_path: &str) -> Result<(), ()>{
    if let Ok(file_data) = std::fs::read(file_path){
        let input_string = String::from_utf8(file_data).unwrap();

        let data = compress(input_string);
        let bytes = bincode::serialize(&data).unwrap();

        if let Ok(_) = std::fs::write(output_path, bytes) {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub fn decompress(data: Data) -> String {
    let mut bits = Vec::with_capacity(data.length);

    let mut k = 0;
    let mut end = false;
    for byte in data.data {
        for i in (0..8).rev() { 
            bits.push((byte >> i) & 1);

            k += 1;

            if k >= data.length {
                end = true;
                break;
            }
        }

        if end {
            break;
        }
    }

    let mut buffer = Vec::<u8>::new();
    let mut output_string = String::new();
    
    for cc in bits.clone() {
        //println!("C: {}", cc);
        buffer.push(cc);
        if buffer.len() > 0 {
            if let Some(char) = find_symbol(data.dict.clone(), buffer.clone(), buffer.len()) {
                output_string.push(char);
                buffer.clear();
            }
        }
    }

    return output_string;
}

pub fn decompress_file(file_path: &str, output_path: &str) -> Result<(), ()>{
    if let Ok(file_data) = std::fs::read(file_path) {
        let data = bincode::deserialize(&file_data).unwrap();
        let data = decompress(data);

        if let Ok(_) = std::fs::write(output_path, data) {
            Ok(())
        } else {
            Err(())
        }      
    } else {
        Err(())
    }
}