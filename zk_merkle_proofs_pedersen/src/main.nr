// Import the Merkle root calculation function
use dep::std;

global MERKLE_DEPTH = 3;

// The main function of the Noir program
fn main(
    leaf_index: Field, // Index of the leaf being updated
    old_leaf: Field, // Old leaf value before update
    new_leaf: Field, // New leaf value after update
    path: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path
    old_root: pub Field, // Old Merkle root (public input)
    new_root: pub Field, // New Merkle root (public input)
) {
    // Initialize hash variables for old and new roots
    let old_hash = std::merkle::compute_merkle_root(old_leaf, leaf_index, path);
    let new_hash = std::merkle::compute_merkle_root(new_leaf, leaf_index, path);

    // Enforce that the computed old root matches the provided old root
    assert(old_hash == old_root);

    // Enforce that the computed new root matches the provided new root
    assert(new_hash == new_root);
}
