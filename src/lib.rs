pub mod engine {
    pub mod data;
}

use engine::data::Data;

const SEPARATOR: char = ' ';

pub fn compress(buf: &[u8]) -> Data {
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

    // println!("Original length: {}", buf.len());
    // let encoded = data.to_bytes();
    // println!("Compress length: {}", encoded.len());

    // std::fs::write(path, encoded).unwrap();
    
    return data;
}

pub fn compress_file(file_path: &str, output_path: &str) -> Result<(), ()>{
    if let Ok(file_data) = std::fs::read(file_path){
        let data = compress(file_data.as_slice());

        if let Ok(_) = std::fs::write(output_path, data.to_bytes()) {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub fn decompress(bytes: &[u8]) -> Data {
    return Data::from_bytes(bytes);
}

pub fn decompress_file(file_path: &str, output_path: &str) -> Result<(), ()>{
    if let Ok(file_data) = std::fs::read(file_path) {
        let data = decompress(file_data.as_slice());
        if let Ok(_) = std::fs::write(output_path, data.to_str()) {
            Ok(())
        } else {
            Err(())
        }      
    } else {
        Err(())
    }
}