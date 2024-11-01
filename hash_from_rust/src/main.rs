use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
struct MerkleTree {
    leaf_nodes: Vec<String>, // Stores hash of each leaf node
    tree: Vec<Vec<String>>,  // Stores the entire Merkle tree layers
}

impl MerkleTree {
    // Creates a new Merkle tree from the given leaf data
    fn new(leaf_data: Vec<String>) -> Self {
        let leaf_nodes: Vec<String> = leaf_data;

        let mut tree = vec![leaf_nodes.clone()];

        // Build the tree level by level
        while tree.last().unwrap().len() > 1 {
            let prev_level = tree.last().unwrap();
            let mut new_level = Vec::new();

            // Hash pairs of nodes from the previous level to create the current level
            for pair in prev_level.chunks(2) {
                let hash = if pair.len() == 2 {
                    compute_pedersen_hash2(&pair[0], &pair[1]).expect("Hashing failed")
                } else {
                    pair[0].clone() // Odd node case
                };
                new_level.push(hash);
            }

            tree.push(new_level);
        }

        Self { leaf_nodes, tree }
    }

    // Retrieves the Merkle root of the tree
    fn root(&self) -> &String {
        self.tree.last().unwrap().first().unwrap()
    }

    // Retrieves the Merkle path proof for a given leaf index
    fn merkle_path(&self, index: usize) -> Vec<String> {
        let mut path = Vec::new();
        let mut idx = index;

        for level in &self.tree[..self.tree.len() - 1] {
            // Exclude root level
            let sibling_index = if idx % 2 == 0 { idx + 1 } else { idx - 1 };

            if sibling_index < level.len() {
                path.push(level[sibling_index].clone());
            }
            idx /= 2;
        }

        path
    }

    // Updates a leaf node and recomputes the affected nodes up to the root
    fn update_leaf(&mut self, index: usize, new_data: &str) {
        // Update the leaf node with the new hash
        self.leaf_nodes[index] = new_data.to_string();
        self.tree[0][index] = self.leaf_nodes[index].clone();

        // Recompute hashes up the tree
        let mut idx = index;
        for i in 0..self.tree.len() - 1 {
            let parent_idx = idx / 2;
            let left_child = &self.tree[i][parent_idx * 2];
            let right_child = if parent_idx * 2 + 1 < self.tree[i].len() {
                &self.tree[i][parent_idx * 2 + 1]
            } else {
                left_child
            };
            self.tree[i + 1][parent_idx] =
                compute_pedersen_hash2(left_child, right_child).expect("Hashing failed");
            idx = parent_idx;
        }
    }
}

fn compute_pedersen_hash2(field_element1: &str, field_element2: &str) -> Result<String, String> {
    // Step 1: Populate Prover.toml
    let toml_content = format!(
        "\ninput1 = \"{}\"\ninput2 = \"{}\"",
        field_element1, field_element2
    );

    let mut file = File::create("../pedersen2/Prover.toml").expect("Unable to create Prover.toml");
    file.write_all(toml_content.as_bytes())
        .expect("Unable to write to Prover.toml");

    // Step 2: Run Noir's `nargo` command and capture output
    let output = Command::new("nargo")
        .args(&["execute", "-p", "Prover", "--program-dir", "../pedersen2/"])
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output() // Using output() to capture stdout and stderr directly
        .expect("Failed to execute Noir program");

    // Check if nargo command executed successfully
    if !output.status.success() {
        return Err(format!(
            "nargo command failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Step 3: Read and parse Noir's output to extract hash
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("0x") {
            return Ok(line.trim().to_string()); // Return the hash if found
        }
    }

    Err("Hash output not found".to_string())
}

// fn main() {
//     // Example usage
//     let field_element1 = "1234567890"; // Replace with actual first field element
//     let field_element2 = "9876543210"; // Replace with actual second field element

//     match compute_pedersen_hash(field_element1, field_element2) {
//         Ok(hash) => println!("Pedersen Hash: {}", hash),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }

fn main() {
    // Example usage
    let leaves = vec![
        "1234".to_string(),
        "2345".to_string(),
        "7545".to_string(),
        "4564".to_string(),
    ];

    // Create a Merkle tree
    let mut merkle_tree = MerkleTree::new(leaves);

    // Print the Merkle root
    println!("Merkle Root: {}", merkle_tree.root());

    // Get Merkle path for the first leaf
    let merkle_path = merkle_tree.merkle_path(0);
    println!("Merkle Path for leaf 0: {:?}", merkle_path);

    // Update a leaf and recompute the tree
    merkle_tree.update_leaf(0, "63453");
    println!("Updated Merkle Root: {}", merkle_tree.root());
}
