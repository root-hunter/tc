use engine::{Data, Engine};

pub mod engine;

pub fn compress(buf: &[u8]) {
    let mut data = Data::<String>::new(buf.len());

    let str = std::str::from_utf8(buf).unwrap().split(' ');
    
    for part in str {
        let part = part.to_string();
        data.add_element(&part);
    }

    println!("Original length: {}", data.size);
    let encoded = data.to_bytes();
    println!("Compress length: {}", encoded.len());
}

pub fn decompress(buf: &[u8]) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let test_path = "/home/roothunter/Dev/ran/files/text1.txt";
        let test = std::fs::read(test_path).unwrap();
        compress(test.as_slice());
    }
}
