# AGENTS.md

## Workspace structure

Cargo workspace with two binary crates under `bin/`:
- `bin/client` — Bevy 0.15 game client (workspace dep)
- `bin/server` — server binary (no deps yet)

No shared library crate. Both are entrypoints.

## Toolchain

Edition 2024 + resolver 3 requires **Rust 1.85+**. No `rust-toolchain` file; CI uses latest stable via `dtolnay/rust-toolchain@stable`.

## Verification (follow CI order)

```sh
cargo fmt --check
cargo check
cargo build
cargo test
cargo clippy -- -D warnings
```

`ci.sh` runs the full pipeline. Pass `--fix` to auto-format instead of checking.

The `-D warnings` flag is required — CI treats all clippy warnings as errors.

**После любых изменений кода запускай проверки** (`ci.sh` или пошагово из блока выше) — прежде чем сообщать о завершении задачи.

## Formatting

No `rustfmt.toml`. Default `cargo fmt` applies.

## Code conventions

- **Документация на русском**: doc-comments и пояснения пишутся на русском. Не добавлять банальные комментарии, дублирующие очевидную логику.
- **Unit-тесты обязательны**: любой новый нетривиальный код должен сопровождаться unit-тестами через `#[cfg(test)] mod tests` в том же crate.

## Notes

- If adding a shared crate, create it as a workspace member and add to `Cargo.toml` `[workspace] members`.
