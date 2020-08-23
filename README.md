# Rust Tools

> Various utilities (re)written in Rust

## Usage

Build dsha256 (example):

```sh
cd dsha256
cargo build
```

Example:

```sh
echo 0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c | xxd -ps -r | ./target/debug/dsha256

6fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000
```

## License

[ISC](LICENSE)
