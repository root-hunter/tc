use chrono::Local;
use env_logger::Builder;
use log::{info, LevelFilter};
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

        Builder::new()
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
            .init();
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
        info!("Original path: {}", original_path);
        info!("Compressed path: {}", compressed_path);
        info!("Decompressed path: {}", decompressed_path);

        let test = std::fs::read(original_path).unwrap();
        tc::compress(compressed_path, test.as_slice());

        let compressed_data = std::fs::read(compressed_path).unwrap();
        let compressed_data = compressed_data.as_slice();

        let decompressed = tc::decompress(&compressed_data);

        std::fs::write(decompressed_path, decompressed).unwrap();

        assert!(are_files_equal(original_path, decompressed_path).unwrap());
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

        info!("Start TEST 1");

        let test = std::fs::read(test_path).unwrap();
        tc::compress(compressed_path, test.as_slice());

        let compressed = std::fs::read(compressed_path).unwrap();
        assert!(compressed.len() < test.len());
    }

    #[test]
    fn test_2() {
        init();

        let original_path = "./tests/inputs/input_1.txt";
        let compressed_path = "./tests/tmp/compressed_2.tc";
        let decompressed_path = "./tests/tmp/decompressed_2.tc";

        info!("Start TEST 2");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_3() {
        init();

        let original_path = "./tests/inputs/input_2.txt";
        let compressed_path = "./tests/tmp/compressed_3.tc";
        let decompressed_path = "./tests/tmp/decompressed_3.tc";

        info!("Start TEST 3");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_4() {
        init();

        let original_path = "./tests/inputs/input_3.txt";
        let compressed_path = "./tests/tmp/compressed_4.tc";
        let decompressed_path = "./tests/tmp/decompressed_4.tc";

        info!("Start TEST 4");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_5() {
        init();

        let original_path = "./tests/inputs/input_4.txt";
        let compressed_path = "./tests/tmp/compressed_5.tc";
        let decompressed_path = "./tests/tmp/decompressed_5.tc";

        info!("Start TEST 5");

        test_file(original_path, compressed_path, decompressed_path);
    }

    #[test]
    fn test_6() {
        init();

        let original_path = "./tests/inputs/input_5.txt";
        let compressed_path = "./tests/tmp/compressed_6.tc";
        let decompressed_path = "./tests/tmp/decompressed_6.tc";

        info!("Start TEST 6");

        test_file(original_path, compressed_path, decompressed_path);
    }
}
