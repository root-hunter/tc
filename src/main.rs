use std::collections::HashMap;

use regex::Regex;

const SEPARATORS: &[char] = &[
    '^', ' ', '\n', '\t', ':', '.', ',', '?', '!', '|', '<', '>', '{', '}', '(', ')', '"', '\'',
    '\\', '/', '&',
];

fn test() {
    let original_path = "./tests/inputs/input_1.txt";
    let compressed_path = "./tests/tmp/compressed_2.tc";
    let decompressed_path = "./tests/tmp/decompressed_2.tc";

    println!("#######################################");

    println!(
        "Original size: {}",
        std::fs::metadata(original_path).unwrap().len()
    );

    let compressed = tc::compress_file(original_path, compressed_path);
    if compressed.is_ok() {
        println!(
            "Compressed size: {}",
            std::fs::metadata(compressed_path).unwrap().len()
        );

        let decompressed = tc::decompress_file(compressed_path, decompressed_path);
        if decompressed.is_ok() {
            println!(
                "Decompress size: {}",
                std::fs::metadata(decompressed_path).unwrap().len()
            );
        } else {
            println!("ERRORORO");
        }
    } else {
        println!("ERRORORO");
    }

    println!("Original path: {}", original_path);
    println!("Compressed path: {}", compressed_path);
    println!("Decompressed path: {}", decompressed_path);
}

fn main() {
    let original_path = "./tests/inputs/input_2.txt";
    let compressed_path = "./tests/tmp/compressed_2.tc";
    let decompressed_path = "./tests/tmp/decompressed_2.tc";

    let reg_word_pattern = r#"^([^ \n\t:.,?|!:;\-_\(\)\[\]\{\}\/%$<>+*\^'"]{2,})$"#;
    let reg_word = Regex::new(reg_word_pattern).unwrap();

    let mut distribution: HashMap<String, usize> = HashMap::new();

    let data = std::fs::read(original_path).unwrap();
    let data = String::from_utf8(data).unwrap();

    let chars = data.chars();

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

    println!("{:#?}", distribution);
}
