// Import the Poseidon hash function
use dep::std::hash::poseidon::bn254;

fn main(a: Field, b: Field) -> pub Field {
    // Compute the hash of two field elements
    let hash_result = bn254::hash_2([a, b]);

    // Return the hash as a public output
    hash_result
}
