TIMESTAMP := $(shell date +%Y%m%d_%H%M%S)

test:
	cargo test --release -v -- --nocapture --test-threads=1 | tee ./tests/history/test_output_${TIMESTAMP}.log