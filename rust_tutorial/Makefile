.PHONY: build
build:
	@echo "Building..."
	cargo build 

.PHONY: run
run:
	@echo "Running..."
	cargo run

.PHONY: lint 
lint:
	@echo "Linting..."
	cargo check

.PHONY: clean
clean:
	@echo "Cleaning..."
	cargo clean

.PHONY: test
test: 
ifdef t 
	@echo "Running test $(t)"
	cargo test $(t) -- --nocapture
else
	@echo "Running tests..."
	cargo test
endif
