# MerkleTree

Library to compute merkle tree root.

There are two implementations - one in Elixir, one in Rust as a NIF.

## Running tests

* First run `mix deps.get`.

* Run unit tests via `mix test`.

* For benchmark tests, first generate the performance data via `make perf-data`
  (it can take a long time). Then run `mix run ./perf_test.exs`.
