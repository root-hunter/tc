use engine::Data;

pub mod engine;

pub fn compress(path: &str, buf: &[u8]) {
    let mut data = Data::new();
    let str = std::str::from_utf8(buf).unwrap();
    
    let mut token = Vec::<u16>::new();
    let mut sep_count = 0;

    let chars = str.chars();
    let chars_count = chars.clone().count();

    println!("COUNT: {}", chars_count);
    let mut i = 0;
    for c in chars {
        if c != ' ' {
            if sep_count > 1 {
                data.add_separator(sep_count);
                sep_count = 0;
            }
            
            token.push(c as u16);

            if i == chars_count - 1 {
                let part = String::from_utf16(token.as_slice()).unwrap();
                data.add_element(&part);
                token.clear();
                sep_count = 0;
            }
        } else if c == ' ' {
            if token.len() > 0 {
                let part = String::from_utf16(token.as_slice()).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_1() {
        let test_path = "/home/roothunter/Dev/ran/files/text_1.txt";
        let compress_path = "/home/roothunter/Dev/ran/files/compress_2.ran";
        let decompress_path = "/home/roothunter/Dev/ran/files/decompress_2.ran";
        
        let test = std::fs::read(test_path).unwrap();
        compress(compress_path, test.as_slice());

        let compress_data = std::fs::read(compress_path).unwrap();
        let compress_data = compress_data.as_slice();

        let decompressed = decompress(&compress_data);

        std::fs::write(decompress_path, decompressed).unwrap();
    }

    #[test]
    fn compress_1() {
        let test_path = "/home/roothunter/Dev/ran/files/text_1.txt";
        let compress_path = "/home/roothunter/Dev/ran/files/compress_1.ran";
        
        let test = std::fs::read(test_path).unwrap();
        compress(compress_path, test.as_slice());
    }
}
