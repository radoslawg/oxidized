# Detect OS
ifeq ($(OS),Windows_NT)
		shell := cmd.exe
    # Windows-specific logic
    # Convert current directory to Windows format and append \extern
		mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
    LIBCLANG_PATH := $(dir $(mkfile_path))extern
else
    # Linux / macOS logic
    LIBCLANG_PATH :=
endif

.PHONY: build run test coverage clean

# The main build command
build:
ifeq ($(OS),Windows_NT)
	@echo "Setting LIBCLANG_PATH to: $(LIBCLANG_PATH)"
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo build
else
	cargo build
endif

# Run the project
run:
ifeq ($(OS),Windows_NT)
	@echo "Setting LIBCLANG_PATH to: $(LIBCLANG_PATH)"
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo run
else
	cargo run
endif

test:
ifeq ($(OS),Windows_NT)
	@echo "Setting LIBCLANG_PATH to: $(LIBCLANG_PATH)"
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo test
else
	cargo test
endif

coverage:
ifeq ($(OS),Windows_NT)
	@echo "Setting LIBCLANG_PATH to: $(LIBCLANG_PATH)"
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo tarpaulin --out Html
else
	 setarch $(shell uname -m) -R cargo tarpaulin --out Html
endif

# Clean the project
clean:
	cargo clean
