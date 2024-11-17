pub mod engine {
    pub mod data;
}

use engine::data::Data;

const SEPARATOR: char = ' ';

pub fn compress(path: &str, buf: &[u8]) {
    
    let mut data = Data::new();
    let str = std::str::from_utf8(buf).unwrap();
    
    let mut token = Vec::<char>::new();
    let mut sep_count = 0;

    let chars = str.chars();
    let chars_count = chars.clone().count();

    println!("COUNT: {}", chars_count);
    let mut i = 0;
    for c in chars {
        if c != SEPARATOR {
            if sep_count > 1 {
                data.add_separator(sep_count);
                sep_count = 0;
            }
            
            token.push(c);

            if i == chars_count - 1 {
                let part = token.clone().into_iter().collect();
                data.add_element(&part);
                token.clear();
                sep_count = 0;
            }
        } else if c == SEPARATOR {
            if token.len() > 0 {
                let part = token.clone().into_iter().collect();
                //println!("PART: {}", part);

                data.add_element(&part);
                token.clear();
                sep_count = 0;
            }
            sep_count += 1;
        }

        i += 1;
    }

    println!("Original length: {}", buf.len());
    let encoded = data.to_bytes();
    println!("Compress length: {}", encoded.len());

    std::fs::write(path, encoded).unwrap();
}

pub fn decompress(bytes: &[u8]) -> String {
    let data: Data = Data::from_bytes(bytes);
    return data.decompress();
}