# List available commands
default:
    @just --list

# Build in debug mode
build:
    cargo build

# Run all tests
test:
    cargo test

# Run clippy linter
lint:
    cargo clippy

# Format code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt --check

# Run in the foreground (debug build)
run:
    cargo run -- run

# Run all checks (test, lint, format)
check: test lint fmt-check

# Publish a new release: crates.io → git tag → GitHub push
release: check
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(cargo metadata --format-version=1 --no-deps | jq -r '.packages[0].version')
    echo "Publishing v${version}"

    cargo publish
    echo "Published to crates.io"

    git tag "v${version}"
    git push
    git push --tags
    echo "Pushed v${version} to GitHub"
