.PHONY: build
build:
	@echo "Building..."
	cargo build 

.PHONY: run
run:
	@echo "Running..."
	cargo run

.PHONY: check 
check:
	@echo "Linting..."
	cargo check

.PHONY: clean
clean:
	@echo "Cleaning..."
	cargo clean

.PHONY: test
test: 
# t is the test name, try to make it unique
ifdef t 
	@echo "Running test $(t)"
	cargo test $(t) -- --nocapture
else
	@echo "Running all tests..."
	cargo test
endif
