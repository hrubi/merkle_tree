defmodule MerkleTreeRust do
  use Rustler, otp_app: :merkle_tree, crate: "merkle_tree"

  def root(_filename), do: :erlang.nif_error(:nif_not_loaded)
end
