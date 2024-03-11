use sha2::{Digest, Sha256};

pub fn meets_difficulty (hash: &str, difficulty: usize) -> bool {
    let zeroes = "0".repeat(difficulty);
    hash.starts_with(&zeroes)
}

pub fn guess (input: &str, nonce: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", input, nonce));
    let result = hasher.finalize();

    hex::encode(result)
}