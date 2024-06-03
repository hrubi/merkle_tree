filename = Path.join(__DIR__, "input-perf.txt")

Benchee.run(%{
  "elixir" => fn -> MerkleTree.root_elixir(filename) end,
  "rust" => fn -> MerkleTree.root_rust(filename) end
})
