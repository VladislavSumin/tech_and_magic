#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--fix" ]]; then
  echo "==> cargo fmt"
  cargo fmt
else
  echo "==> cargo fmt --check"
  cargo fmt --check
fi

echo "==> cargo check"
cargo check

echo "==> cargo build"
cargo build

echo "==> cargo test"
cargo test

echo "==> cargo clippy -- -D warnings"
cargo clippy -- -D warnings

echo "All checks passed!"
