#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn are_files_equal(file1: &str, file2: &str) -> std::io::Result<bool> {
        let mut file1 = File::open(file1)?;
        let mut file2 = File::open(file2)?;

        let mut file1_contents = Vec::new();
        let mut file2_contents = Vec::new();

        file1.read_to_end(&mut file1_contents).unwrap();
        file2.read_to_end(&mut file2_contents).unwrap();

        Ok(file1_contents == file2_contents)
    }

    fn test_file(original_path: &str, compressed_path: &str, decompressed_path: &str) {
        println!("#######################################");

        println!("Original size: {}", std::fs::metadata(original_path).unwrap().len());

        let compressed = tc::compress_file(original_path, compressed_path);
        if compressed.is_ok() {
            println!("Compressed size: {}", std::fs::metadata(compressed_path).unwrap().len());

            let decompressed = tc::decompress_file(compressed_path, decompressed_path);
            if decompressed.is_ok() {
                println!("Decompress size: {}", std::fs::metadata(decompressed_path).unwrap().len());

                assert!(are_files_equal(original_path, decompressed_path).unwrap());
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }

        println!("Original path: {}", original_path);
        println!("Compressed path: {}", compressed_path);
        println!("Decompressed path: {}", decompressed_path);
    }

    #[test]
    fn test_1() {
        let original_path = "./tests/inputs/input_1.txt";
        let compressed_path = "./tests/tmp/compressed_1.tc";
        let decompressed_path = "./tests/tmp/decompressed_1.tc";

        println!("Start TEST 1");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_2() {
        let original_path = "./tests/inputs/input_2.txt";
        let compressed_path = "./tests/tmp/compressed_2.tc";
        let decompressed_path = "./tests/tmp/decompressed_2.tc";

        println!("Start TEST 2");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_3() {
        let original_path = "./tests/inputs/input_3.txt";
        let compressed_path = "./tests/tmp/compressed_3.tc";
        let decompressed_path = "./tests/tmp/decompressed_3.tc";

        println!("Start TEST 3");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_4() {
        let original_path = "./tests/inputs/input_4.txt";
        let compressed_path = "./tests/tmp/compressed_4.tc";
        let decompressed_path = "./tests/tmp/decompressed_4.tc";

        println!("Start TEST 4");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_5() {
        let original_path = "./tests/inputs/input_5.txt";
        let compressed_path = "./tests/tmp/compressed_5.tc";
        let decompressed_path = "./tests/tmp/decompressed_5.tc";

        println!("Start TEST 5");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_6() {
        let original_path = "./tests/inputs/input_6.txt";
        let compressed_path = "./tests/tmp/compressed_6.tc";
        let decompressed_path = "./tests/tmp/decompressed_6.tc";

        println!("Start TEST 6");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_7() {
        let original_path = "./tests/inputs/input_7.txt";
        let compressed_path = "./tests/tmp/compressed_7.tc";
        let decompressed_path = "./tests/tmp/decompressed_7.tc";

        println!("Start TEST 7");

        test_file(original_path, compressed_path, decompressed_path);
    }
}
