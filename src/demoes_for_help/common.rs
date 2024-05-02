use blake2::{Blake2s, Digest};

fn hash_number(number: u64) -> [u8; 32] {
    // Create a Blake2s hasher
    let mut hasher = Blake2s::new();

    // Update the hasher with the bytes of the input number
    hasher.update(&number.to_le_bytes());

    // Finalize the hash and return the result as a fixed-size array
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_number() {
        // Test case 1
        let number1 = 12345;
        let hash1: [u8; 32] = hash_number(number1);
        assert_eq!(hash1.len(), 32); // Ensure the hash has the correct length

        // Test case 2
        let number2 = 98765;
        let hash2 = hash_number(number2);

        // Assert that the hashes for different numbers are not equal
        assert_ne!(hash1, hash2);
    }
}