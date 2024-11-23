fn main() {
  let original_path = "./tests/inputs/input_1.txt";
  let compressed_path = "./tests/tmp/compressed_2.tc";
  let decompressed_path = "./tests/tmp/decompressed_2.tc";

  println!("#######################################");

  println!("Original size: {}", std::fs::metadata(original_path).unwrap().len());

  let compressed = tc::compress_file(original_path, compressed_path);
  if compressed.is_ok() {
      println!("Compressed size: {}", std::fs::metadata(compressed_path).unwrap().len());

      let decompressed = tc::decompress_file(compressed_path, decompressed_path);
      if decompressed.is_ok() {
          println!("Decompress size: {}", std::fs::metadata(decompressed_path).unwrap().len());
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