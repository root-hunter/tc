use std::collections::{BTreeMap, HashMap};
use priority_queue::{self, PriorityQueue};
use serde::de::value;
use std::cmp::Reverse;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    pub is_root: bool,
    pub freq: usize,
    pub symbol: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

struct DataV2 {
    dict: BTreeMap<usize, Node>
}

fn build_dict(node: Node, dict: &mut HashMap<String, char>, acc: String, depth: usize, from_left: bool) {
    if node.left == None && node.right == None {
        let mut bit  = 1;

        if from_left {
            bit = 0;
        }

        //let key = format!("{}{}", acc, bit);

        dict.insert(acc, node.symbol.unwrap());
    } else {
        let left = node.left.unwrap();
        let right = node.right.unwrap();

        if !node.is_root && depth > 0 {
            build_dict(*left, dict, format!("{}{}", acc, 0), depth + 1, true);
            build_dict(*right, dict, format!("{}{}", acc, 1), depth + 1, false);
        } else {
            build_dict(*left, dict, "0".to_string(), 1, true);
            build_dict(*right, dict, "1".to_string(), 1, false);
        }

    }
}

fn main() {
    let data = BTreeMap::new();

    data.get(key)

    let input_string = "AABBCDV dsad sad daaaaaaskm mmmf dkwqe qqwmm kmz z zm kmdask dmm ";

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
    
    assert!(distribution.values().sum::<usize>() == chars.count());
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

    let mut dict = HashMap::<String, char>::new();

    build_dict(root, &mut dict, String::new(), 0, true);

    for (k, v) in &dict {
        println!("{}:{}", v, k);
    }

    println!("HASH: {:?}", dict);
}