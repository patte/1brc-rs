# 1BRC in rust

[The One Billion Row Challenge](https://github.com/gunnarmorling/1brc).

Atm this is an attempt with rayon for parallelism.
Improved with inpiration from [this gist by Lucretiel](https://gist.github.com/Lucretiel/b9d8a2f75c445ba62035fd80adb5fd57).
Thanks to [kobzol](https://kobzol.github.io/rust/cargo/2024/01/23/making-rust-binaries-smaller-by-default.html) the binary size is now also quite small `628KB`.

On my MacBook Air (M2, 2023) it takes ~48s to complete. Memory usage peaks at ~14GB.

## Installation
This requires rust nightly (for `slice_split_one`).
```bash
rustup install nightly
rustup override set nightly
```

## Usage

### Memory mapped file, delayed string parsing, I48F16
```bash
cargo build --release && time target/release/one_billion_row_challenge_rayon
Abha;18/-33.9/67.5
Abidjan;26/-24.1/74.6
...
Zanzibar City;26/-26.1/77
Zürich;9.3/-38.2/57.5
Ürümqi;7.4/-43.5/55.7
İzmir;17.9/-36.6/64.3
Number of stations: 413
target/release/one_billion_row_challenge_rayon  47.19s user 32.38s system 241% cpu 32.908 total
```

### Just rayon
MacBook Air (M2, 2023): ~132s to complete, memory usage peaks at ~20GB.

```bash
cargo build --release && time target/release/one_billion_row_challenge_rayon
    Finished release [optimized] target(s) in 0.00s
Abha;-33.9/18/67.5
Abidjan;-24.1/26/74.6
...
Zanzibar City;-26.1/26/77
Zürich;-38.2/9.3/57.5
Ürümqi;-43.5/7.4/55.7
İzmir;-36.6/17.9/64.3
Number of stations: 413
target/release/one_billion_row_challenge_rayon  134.69s user 17.30s system 409% cpu 37.141 total
```