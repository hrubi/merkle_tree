# Merkle Tree

Application and library to compute merkle tree root.

This is implementation in Rust. The code for the root computation is in
[lib.rs](src/lib.rs) and there is a tiny wrapper [main.rs](src/main.rs) for the
binary.

There is also an
[implementation in Elixir](https://github.com/hrubi/merkle_tree/blob/elixir/lib/merkle_tree.ex).
It is more trivial and was used as a reference implementation to validate
correctness. That [branch](https://github.com/hrubi/merkle_tree/tree/elixir)
also contains a Rust implementation as a NIF.

## Running the binary

* Build with `cargo build -r`.

* Run with `./target/release/merkle_tree fixtures/input.txt`.

## Running tests

* Run unit tests via `cargo test`.

* For benchmark tests, first generate the performace data via `make perf-data`
  (it can take a long time). Then run `cargo bench`.
