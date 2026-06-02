# AGENTS.md

## Workspace structure

Cargo workspace with two binary crates under `bin/`:
- `bin/client` — client binary
- `bin/server` — server binary

No shared library crate. Both are entrypoints.

## Toolchain

Edition 2024 + resolver 3 requires **Rust 1.85+**. No `rust-toolchain` file is pinned; CI uses latest stable via `dtolnay/rust-toolchain@stable`.

## Verification (follow CI order)

```sh
cargo check
cargo build
cargo test
cargo clippy -- -D warnings
```

The `-D warnings` flag is required — CI treats all clippy warnings as errors. Omitting it will pass locally but fail in CI.

## Formatting

No `rustfmt.toml`. Default `cargo fmt` applies.

## Code conventions

- **Документация на русском**: подробные doc-comments и пояснения пишутся на русском языке. Не добавлять банальные комментарии, дублирующие очевидную логику.
- **Unit-тесты обязательны**: любой новый нетривиальный код должен сопровождаться unit-тестами. Тесты размещать в том же crate через `#[cfg(test)] mod tests`.

## Notes

- No shared crate yet. If adding one, create it as a workspace member and add to `Cargo.toml` `[workspace] members`.
