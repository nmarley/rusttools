# Rust Tools

> Various utilities (re)written in Rust

## Install

From the repo root, install any tool into `~/.cargo/bin`:

```sh
cargo install --path <tool>
```

Examples:

```sh
cargo install --path finddupes
cargo install --path rando
cargo install --path lc
```

Reinstall after changes with `--force`:

```sh
cargo install --path finddupes --force
```

List workspace members with `cargo metadata --no-deps --format-version 1` or by looking at the root `Cargo.toml`.

## Build

```sh
cargo build -p <tool>
```

Example (`dsha256`):

```sh
cargo build -p dsha256
echo 0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c | xxd -ps -r | ./target/debug/dsha256

6fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000
```

## License

[ISC](LICENSE)
