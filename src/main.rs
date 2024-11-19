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

fn build_btree(node: Node, dict: &mut HashMap<Box<Vec<u8>>, char>, acc: Box<Vec<u8>>, depth: usize) {
    if node.left == None && node.right == None {
        //let key = format!("{}{}", acc, bit);

        dict.insert(acc, node.symbol.unwrap());
    } else {
        let left = node.left.unwrap();
        let right = node.right.unwrap();

        if !node.is_root && depth > 0 {
            let mut acc_left = *acc.clone();
            acc_left.push(0);

            let mut acc_rigth = *acc.clone();
            acc_rigth.push(1);

            build_btree(*left, dict, Box::new(acc_left), depth + 1);
            build_btree(*right, dict, Box::new(acc_rigth), depth + 1);
        } else {
            let acc_left = Box::new(Vec::from([0]));
            let acc_rigth = Box::new(Vec::from([1]));

            build_btree(*left, dict, acc_left, 1);
            build_btree(*right, dict, acc_rigth, 1);
        }
    }
}

fn main() {
    let data = BTreeMap::<u16, char>::new();


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

    let mut dict = HashMap::<Box<Vec<u8>>, char>::new();
    let acc = Box::new(Vec::new());

    build_btree(root, &mut dict, acc, 0);

    for (k, v) in &dict {
        println!("{}:{:?}", v, *k);
    }

    println!("HASH: {:?}", dict);
}