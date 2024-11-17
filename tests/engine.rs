use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

static IS_LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);
static INIT_LOGGER_ONCE: Once = Once::new();

fn init_logger() {
    INIT_LOGGER_ONCE.call_once(|| {
        let timestamp = Local::now().format("%Y_%m_%d");
        let log_path = format!("./tests/logs/test_{}.log", timestamp);

        let target = Box::new(File::create(log_path).expect("Can't create file"));

        let result  = Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}:{} {} [{}] - {}",
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                    record.level(),
                    record.args()
                )
            })
            .target(env_logger::Target::Pipe(target))
            .filter(None, LevelFilter::Info)
            .try_init();

        if result.is_ok() {
            println!("Logger init with success");
        } else {
            println!("Error to init logger");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    use crate::init_logger;

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
        println!("Original path: {}", original_path);

        println!("Compressed path: {}", compressed_path);
        println!("Decompressed path: {}", decompressed_path);

        let compressed = tc::compress_file(original_path, compressed_path);
        if compressed.is_ok() {
            println!("Original size: {}", std::fs::metadata(original_path).unwrap().len());
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
    }

    fn is_init() -> bool {
        return IS_LOGGER_INITIALIZED.load(Ordering::SeqCst);
    }

    fn init() {
        if !is_init() {
            init_logger();
            IS_LOGGER_INITIALIZED.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_1() {
        init();

        let test_path = "./tests/inputs/input_1.txt";
        let compressed_path = "./tests/tmp/compressed_1.tc";

        println!("Start TEST 1");

        tc::compress_file(test_path, compressed_path).unwrap();
        let test = std::fs::read(test_path).unwrap();
        let compressed = std::fs::read(compressed_path).unwrap();

        assert!(compressed.len() < test.len());
    }

    #[test]
    fn test_2() {
        init();

        let original_path = "./tests/inputs/input_1.txt";
        let compressed_path = "./tests/tmp/compressed_2.tc";
        let decompressed_path = "./tests/tmp/decompressed_2.tc";

        println!("Start TEST 2");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_3() {
        init();

        let original_path = "./tests/inputs/input_2.txt";
        let compressed_path = "./tests/tmp/compressed_3.tc";
        let decompressed_path = "./tests/tmp/decompressed_3.tc";

        println!("Start TEST 3");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_4() {
        init();

        let original_path = "./tests/inputs/input_3.txt";
        let compressed_path = "./tests/tmp/compressed_4.tc";
        let decompressed_path = "./tests/tmp/decompressed_4.tc";

        println!("Start TEST 4");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_5() {
        init();

        let original_path = "./tests/inputs/input_4.txt";
        let compressed_path = "./tests/tmp/compressed_5.tc";
        let decompressed_path = "./tests/tmp/decompressed_5.tc";

        println!("Start TEST 5");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_6() {
        init();

        let original_path = "./tests/inputs/input_5.txt";
        let compressed_path = "./tests/tmp/compressed_6.tc";
        let decompressed_path = "./tests/tmp/decompressed_6.tc";

        println!("Start TEST 6");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_7() {
        init();

        let original_path = "./tests/inputs/input_7.txt";
        let compressed_path = "./tests/tmp/compressed_7.tc";
        let decompressed_path = "./tests/tmp/decompressed_7.tc";

        println!("Start TEST 6");

        test_file(original_path, compressed_path, decompressed_path);
    }
}
