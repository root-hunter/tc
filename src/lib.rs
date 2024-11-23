use std::{cmp::Reverse, collections::HashMap};
use regex::Regex;

use engine::core::{build_dict, find_symbol, Data, Node};
use priority_queue::PriorityQueue;

const SEPARATORS: &[char] = &[
    '^', ' ', '\n', '\t', ':', '.', ',', '?', '!', '|', '<', '>', '{', '}', '(', ')', '"', '\'',
    '\\', '/', '&',
];

pub mod engine {
    pub mod core;
}

pub fn compress(input_string: String) -> Data {
    let chars = input_string.chars();

    let mut distribution = HashMap::<String, usize>::new();

    let reg_word_pattern = r#"^([^ \n\t:.,?|!:;\-_\(\)\[\]\{\}\/%$<>+*\^'"]{2,})$"#;
    let reg_word = Regex::new(reg_word_pattern).unwrap();

    let mut distribution: HashMap<String, usize> = HashMap::new();

    let chars = input_string.chars();
    let mut buffer = String::new();

    for ch in chars {
        if SEPARATORS.contains(&ch) {
          if buffer.len() > 0 {
                if reg_word.is_match(&buffer) {
                    if let Some(elem) = distribution.get_mut(&buffer) {
                        *elem += 1;
                    } else {
                        distribution.insert(buffer.clone(), 1);
                    }
                } else {
                    for x in buffer.chars() {
                        let x = String::from(x);
                        if let Some(elem) = distribution.get_mut(&x) {
                            *elem += 1;
                        } else {
                            distribution.insert(x.clone(), 1);
                        }
                    }
                }
                buffer.clear();
            }

            let ch_str = String::from(ch);
            if let Some(elem) = distribution.get_mut(&ch_str) {
                *elem += 1;
            } else {
                distribution.insert(ch_str.clone(), 1);
            }
        } else {
            buffer.push(ch);
        }
    }

    for (k, v) in distribution.clone() {
      if v < 3 {
        for ch in k.chars() {
          let ch_str = String::from(ch);
          if let Some(elem) = distribution.get_mut(&ch_str) {
              *elem += 1;
          } else {
              distribution.insert(ch_str.clone(), 1);
          }
        }
        
        distribution.remove(&k).unwrap();
      }
    }

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
    println!("{:?}", root);

    // let mut conversion_dict = HashMap::<String, Vec<u8>>::new();
    // let acc = Vec::new();

    // build_dict(root.clone(), &mut conversion_dict, acc, 0, true);
    // let mut compressed: Vec<u8> = Vec::new();

    // for c in chars.clone() {
    //     //println!("{} -> {:?}", c, conversion_dict.get(&c.clone()));
    //     let comp = conversion_dict.get(&c.clone()).unwrap().clone();

    //     for d in comp {
    //         compressed.push(d);
    //     }
    // }
    // let compressed_len = compressed.len();

    // let chunks = compressed.chunks(8);

    // let mut output: Vec<u8> = Vec::new();

    // for chunk in chunks {
    //     let mut b: u8 = 0x00;

    //     let mut position: usize = 0;
    //     for bit in chunk {
    //         if *bit == 1 {
    //             b = b | 1 << 7 - position;
    //         } else {
    //             b = b | 0 << 7 - position;
    //         }

    //         position += 1;
    //     }
    //     output.push(b);
    //     //println!("COMPRESSED CHUNK ({:08b}): {:?}", b, chunk);
    // }

    return Data{
        length: 0,
        dict: root,
        data: Vec::new()
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