defmodule MerkleTree do
  @moduledoc """
  Module for operations on `MerkleTree`.
  """

  @type node_hash :: String.t()

  @spec root_elixir(String.t()) :: node_hash
  def root_elixir(filename) do
    filename
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Enum.map(&:binary.decode_hex/1)
    |> do_root()
  end

  @spec root_rust(String.t()) :: node_hash
  def root_rust(filename) do
    MerkleTreeRust.root(filename)
  end

  @spec do_root([node_hash]) :: node_hash
  defp do_root([root]) do
    for <<c::4 <- root>> do
      elem({?0, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?a, ?b, ?c, ?d, ?e, ?f}, c)
    end
    |> List.to_string()
  end

  defp do_root(nodes) when is_list(nodes) do
    nodes
    |> Enum.chunk_every(2)
    |> Enum.map(&hash_pair/1)
    |> do_root()
  end

  defp hash_pair([h1, h2]), do: sha256(h1 <> h2)
  defp hash_pair([h1]), do: h1

  defp sha256(data) do
    :crypto.hash(:sha256, data)
  end
end
