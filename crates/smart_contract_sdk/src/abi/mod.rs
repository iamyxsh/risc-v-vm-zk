mod tests;

use sha3::{Digest, Keccak256};

pub fn selector(signature: &str) -> u32 {
    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let hash = hasher.finalize();
    u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
}
