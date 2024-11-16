#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Read};

    fn are_files_equal(file1: &str, file2: &str) -> std::io::Result<bool> {
        let mut file1 = File::open(file1)?;
        let mut file2 = File::open(file2)?;
    
        let mut file1_contents = Vec::new();
        let mut file2_contents = Vec::new();
    
        // Read the contents of both files into byte vectors
        file1.read_to_end(&mut file1_contents).unwrap();
        file2.read_to_end(&mut file2_contents).unwrap();
    
        // Compare the contents byte by byte
        Ok(file1_contents == file2_contents)
    }

    fn test_file(original_path: &str, compressed_path: &str, decompressed_path: &str) {
        let test = std::fs::read(original_path).unwrap();
        ran::compress(compressed_path, test.as_slice());

        let compress_data = std::fs::read(compressed_path).unwrap();
        let compress_data = compress_data.as_slice();

        let decompressed = ran::decompress(&compress_data);

        std::fs::write(decompressed_path, decompressed).unwrap();

        assert!(are_files_equal(original_path, decompressed_path).unwrap());
    }

    #[test]
    fn test_1() {
        let test_path = "/home/roothunter/Dev/ran/files/input_1.txt";
        let compressed_path = "/home/roothunter/Dev/ran/files/compress_1.ran";
        
        let test = std::fs::read(test_path).unwrap();
        ran::compress(compressed_path, test.as_slice());

        let compressed = std::fs::read(compressed_path).unwrap();
        assert!(compressed.len() < test.len());
    }

    #[test]
    fn test_2() {
        let original_path = "/home/roothunter/Dev/ran/files/input_1.txt";
        let compressed_path = "/home/roothunter/Dev/ran/files/compress_2.ran";
        let decompressed_path = "/home/roothunter/Dev/ran/files/decompress_2.ran";
        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_3() {
        let original_path = "/home/roothunter/Dev/ran/files/input_2.txt";
        let compressed_path = "/home/roothunter/Dev/ran/files/compress_3.ran";
        let decompressed_path = "/home/roothunter/Dev/ran/files/decompress_3.ran";
        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_4() {
         let original_path = "/home/roothunter/Dev/ran/files/input_3.txt";
         let compressed_path = "/home/roothunter/Dev/ran/files/compress_4.ran";
         let decompressed_path = "/home/roothunter/Dev/ran/files/decompress_4.ran";
         test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_5() {
        let original_path = "/home/roothunter/Dev/ran/files/input_4.txt";
        let compressed_path = "/home/roothunter/Dev/ran/files/compress_5.ran";
        let decompressed_path = "/home/roothunter/Dev/ran/files/decompress_5.ran";
        test_file(original_path, compressed_path, decompressed_path);
    }


    #[test]
    fn test_6() {
        let original_path = "/home/roothunter/Dev/ran/files/input_5.txt";
        let compressed_path = "/home/roothunter/Dev/ran/files/compress_6.ran";
        let decompressed_path = "/home/roothunter/Dev/ran/files/decompress_6.ran";
        test_file(original_path, compressed_path, decompressed_path);
    }


}
