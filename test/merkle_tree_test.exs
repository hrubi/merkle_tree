defmodule MerkleTreeTest do
  use ExUnit.Case
  doctest MerkleTree

  @input_orig Path.join([__DIR__, "..", "input.txt"])
  @input_simple Path.join([__DIR__, "..", "input-simple.txt"])
  @input_odd Path.join([__DIR__, "..", "input-odd.txt"])
  @input_noeol Path.join([__DIR__, "..", "input-noeol.txt"])
  @input_perf Path.join([__DIR__, "..", "input-perf.txt"])
  @simple_root "3a1a4fceddd645502c3bca00e99bbbd63ab4d816bd59d196847eeb9bbee486b6"

  test "original input" do
    assert MerkleTree.root_elixir(@input_orig) == MerkleTree.root_rust(@input_orig)
  end

  test "root_elixir" do
    assert MerkleTree.root_elixir(@input_simple) == @simple_root
  end

  test "root_rust" do
    assert MerkleTree.root_rust(@input_simple) == @simple_root
  end

  test "odd number of lines" do
    assert MerkleTree.root_elixir(@input_odd) == MerkleTree.root_rust(@input_odd)
  end

  test "no EOL" do
    assert MerkleTree.root_elixir(@input_noeol) == MerkleTree.root_rust(@input_noeol)
  end

  @tag :perf
  test "perf dataset" do
    assert MerkleTree.root_elixir(@input_perf) == MerkleTree.root_rust(@input_perf)
  end
end
