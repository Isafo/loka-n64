# loka-n64

## Build Environment Setup

```bash
cargo install cargo-xbuild

git clone https://github.com/JoNil/cargo-n64
cd cargo-n64/cargo-n64
cargo build --release

rustup default nightly
rustup component add rust-src
rustup default stable
```

Place cargo-n64.exe in .cargo/bin

## Extract bootcode

Download mario 64: https://edgeemu.net/details-11757.htm

```bash
cargo run --bin extract_boot_code -- "roms/Super Mario 64 (U) [!].z64"
```

## Build for N64

```bash
cargo +nightly n64 build --ipl3 bootcode.bin --package game
```

## Build for PC

```bash
cargo build --bin game
```

## Links

http://n64dev.org/
https://github.com/n64decomp/sm64/