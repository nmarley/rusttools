# Rust Tools

> Various utilities (re)written in Rust

## Install

Requires [just](https://github.com/casey/just). From the repo root:

```sh
just install
```

That installs every workspace tool into `~/.cargo/bin`. Bare `cargo install` fails here because the root manifest is a virtual workspace.

One tool only:

```sh
just install-one finddupes
```

Or with Cargo directly:

```sh
cargo install --path finddupes
cargo install --path finddupes --force   # after local changes
```

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
