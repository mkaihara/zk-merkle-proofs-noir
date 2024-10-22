// Import the Poseidon hash function, which is suitable for ZK circuits
use dep::std::hash::poseidon::bn254;

global MERKLE_DEPTH = 3;

// The main function of the Noir program
fn main(
    leaf_index: u64, // Index of the leaf being updated
    old_leaf: Field, // Old leaf value before update
    new_leaf: Field, // New leaf value after update
    path: [Field; MERKLE_DEPTH], // Sibling nodes along the Merkle path
    old_root: pub Field, // Old Merkle root (public input)
    new_root: pub Field // New Merkle root (public input)
) {
    // Initialize hash variables for old and new roots
    let mut old_hash = old_leaf;
    let mut new_hash = new_leaf;

    // Iterate over each level of the tree to compute hashes
    for i in 0..MERKLE_DEPTH {
        // Determine if the current node is a left or right child
        let bit = (leaf_index >> i as u8) & 1;

        if bit == 0 {
            // Current node is a left child
            old_hash = bn254::hash_2([old_hash, path[i]]);
            new_hash = bn254::hash_2([new_hash, path[i]]);
        } else {
            // Current node is a right child
            old_hash = bn254::hash_2([path[i], old_hash]);
            new_hash = bn254::hash_2([path[i], new_hash]);
        }
    }

    // Enforce that the computed old root matches the provided old root
    assert(old_hash == old_root);

    // Enforce that the computed new root matches the provided new root
    assert(new_hash == new_root);
}
