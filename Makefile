# Detect OS
ifeq ($(OS),Windows_NT)
		shell := cmd.exe
    # Windows-specific logic
    # Convert current directory to Windows format and append \extern
		mkfile_path := $(abspath $(lastword $(MAKEFILE_LIST)))
    LIBCLANG_PATH := $(dir $(mkfile_path))extern
    EXE_EXT := .exe
else
    # Linux / macOS logic
    LIBCLANG_PATH :=
    EXE_EXT :=
endif

.PHONY: build run test coverage clean

# The main build command
build:
	@echo "Setting LIBCLANG_PATH to: $(LIBCLANG_PATH)"
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo build

# Run the project
run:
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo run

test:
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo test

coverage:
	LIBCLANG_PATH="$(LIBCLANG_PATH)" cargo tarpaulin --out Html

# Clean the project
clean:
	cargo clean
