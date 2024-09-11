# Makefile for Rust Pomodoro Timer

# Variables
CARGO := cargo
BINARY_NAME := pom-rs
TARGET_DIR := target/release

# Phony targets
.PHONY: all build run clean test check fmt lint help

# Default target
all: build

# Build the release version
build:
	@echo "Building release version..."
	$(CARGO) build --release

# Run the application
run: build
	@echo "Running the Pomodoro Timer..."
	./$(TARGET_DIR)/$(BINARY_NAME)

# Clean the project
clean:
	@echo "Cleaning..."
	$(CARGO) clean

# Run tests
test:
	@echo "Running tests..."
	$(CARGO) test

# Check if the project compiles
check:
	@echo "Checking if the project compiles..."
	$(CARGO) check

# Format the code
fmt:
	@echo "Formatting code..."
	$(CARGO) fmt

# Run clippy for linting
lint:
	@echo "Linting code..."
	$(CARGO) clippy

# Help target
help:
	@echo "Makefile commands:"
	@echo "make build    - Build the release version"
	@echo "make run      - Build and run the application"
	@echo "make clean    - Clean the project"
	@echo "make test     - Run tests"
	@echo "make check    - Check if the project compiles"
	@echo "make fmt      - Format the code"
	@echo "make lint     - Run clippy for linting"
	@echo "make help     - Show this help message"
