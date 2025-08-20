.PHONY: help build test clean check fmt lint audit coverage bench fuzz docs release install

# Default target
help:
	@echo "Available targets:"
	@echo "  build      - Build the project in release mode"
	@echo "  test       - Run all tests"
	@echo "  clean      - Clean build artifacts"
	@echo "  check      - Run cargo check"
	@echo "  fmt        - Format code with rustfmt"
	@echo "  lint       - Run clippy linting"
	@echo "  audit      - Run security audit"
	@echo "  coverage   - Generate code coverage report"
	@echo "  bench      - Run benchmarks"
	@echo "  fuzz       - Run fuzz tests (requires nightly)"
	@echo "  docs       - Generate documentation"
	@echo "  release    - Build optimized release binary"
	@echo "  install    - Install the library locally"
	@echo "  all        - Run fmt, lint, test, and audit"

# Build the project
build:
	cargo build --release --all-features

# Run all tests
test:
	cargo test --all-features --verbose
	cargo test --doc --verbose

# Clean build artifacts
clean:
	cargo clean
	rm -rf coverage/
	rm -f lcov.info

# Check code without building
check:
	cargo check --all-features

# Format code
fmt:
	cargo fmt

# Run clippy
lint:
	cargo clippy -- -D warnings

# Security audit
audit:
	cargo audit

# Generate code coverage
coverage:
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "Installing cargo-tarpaulin..."; cargo install cargo-tarpaulin; }
	cargo tarpaulin --all-features --workspace --out Html --out Lcov --output-dir ./coverage

# Run benchmarks
bench:
	cargo bench

# Run fuzz tests
fuzz:
	@command -v cargo-fuzz >/dev/null 2>&1 || { echo "Installing cargo-fuzz..."; cargo +nightly install cargo-fuzz; }
	cd fuzz && cargo +nightly fuzz run fuzz_distribution -- -max_total_time=60

# Generate documentation
docs:
	cargo doc --no-deps --all-features --open

# Build release binary
release:
	cargo build --release --bin sim
	@echo "Release binary built at: target/release/sim"

# Install the library
install:
	cargo install --path .

# Run all checks
all: fmt lint test audit
	@echo "All checks passed!"

# Development setup
dev-setup:
	rustup component add clippy rustfmt
	cargo install cargo-audit cargo-tarpaulin cargo-fuzz
	@echo "Development environment setup complete!"

# Run the simulation
sim:
	cargo run --release --bin sim -- --price-eur-cents 9649600 --cap-eur-cents 1000000 --simulate-steps 5 --claim-step 50

# Quick check before committing
pre-commit: fmt lint test
	@echo "Pre-commit checks passed!"

# Update dependencies
update:
	cargo update
	cargo audit

# Check MSRV (Minimum Supported Rust Version)
msrv:
	cargo +1.77.0 check --all-features
	cargo +1.77.0 test --all-features