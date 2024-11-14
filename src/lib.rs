use engine::Data;

pub mod engine;

pub fn compress(buf: &[u8]) {
    let mut data = Data::new();

    let str = std::str::from_utf8(buf).unwrap();
    
    for b in buf {
        data.add_element(b);
    }

    println!("{:?}", data);
    let encoded = data.encode();
    println!("({}) {:?}", encoded.len(), encoded);
}

pub fn decompress(buf: &[u8]) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let buf = "Ciao mooooooooooooooondo!";
        compress(buf.as_bytes());
    }
}
