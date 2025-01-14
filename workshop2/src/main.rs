/*                                                      
  _____           _                   _   _                 
 |_   _|         | |                 | | (_)                
   | |  _ __  ___| |_ _ __ _   _  ___| |_ _  ___  _ __  ___ 
   | | | '_ \/ __| __| '__| | | |/ __| __| |/ _ \| '_ \/ __|
  _| |_| | | \__ \ |_| |  | |_| | (__| |_| | (_) | | | \__ \
 |_____|_| |_|___/\__|_|   \__,_|\___|\__|_|\___/|_| |_|___/
                                                                                                                     
*/

// First Import necessary crates. Also change the Cargo.toml file
// Necessary libraries: 
// - https://crates.io/crates/sha2
// - https://crates.io/crates/hex
// - https://crates.io/crates/rand
// - https://crates.io/crates/secp256k1
// Then implement functions, pass tests.


/* 
  _______        _        
 |__   __|      | |       
    | | __ _ ___| | _____ 
    | |/ _` / __| |/ / __|
    | | (_| \__ \   <\__ \
    |_|\__,_|___/_|\_\___/
                          
*/

// ---------------------
// Task 1: Hash a message with SHA256, use related hash function in the sha2 crate for hashing. Use hex crate for hex encoding
// Input: &str message
// Output: hex-encoded SHA256 hash string
// Topics: SHA256 hashing, hex encoding
// https://en.wikipedia.org/wiki/SHA-2
// https://stackoverflow.com/questions/1014308/what-is-the-purpose-of-hex-encoding-for-binary-data
fn hash_message(message: &str) -> String {
    // Also print the message, hashed message and hex encoded message to see their format.
    unimplemented!()
}

// ---------------------
// Task 2: Given a message and a known SHA256 hash, verify that hashing the message
// yields the same hash. You can use the hash_message function in the task 1.
// Input: &str message, &str known_hash
// Output: bool (true if matches, false otherwise)
// Topics: hash verification, string comparison
fn verify_hash(message: &str, known_hash: &str) -> bool {
    // KISS for the sake of beginner level problem.
    unimplemented!()
}

// Task 3: Generate a random nonce, use rand crate.
// Concepts: Randomness, Nonces in cryptography
// https://en.wikipedia.org/wiki/Cryptographic_nonce
fn generate_nonce() -> u32 {
    // Also KISS for the sake of beginner level problem.
    unimplemented!()
}

// ---------------------
// Task 4: Generate a random private key and return it as a hex string. Use rand and hex crates.
// Topics: randomness (rand), hex encoding a key
// https://en.wikipedia.org/wiki/Public-key_cryptography
// https://www.geeksforgeeks.org/difference-between-private-key-and-public-key/
fn generate_private_key() -> String {
    // HINT: 256 bits = 32 bytes :P
    // Also print the private key to see what it looks like
    unimplemented!()
}

// ---------------------
// BONUS TASK: Derive a Public Key from a Private Key using the secp256k1 crate 
// Topics: Private key, Public key, Cryptography, Elliptic curve (Secp256k1) 
// Concepts: Public key cryptography, Deterministic derivation from private key
// https://en.wikipedia.org/wiki/Public-key_cryptography
// https://en.wikipedia.org/wiki/Elliptic-curve_cryptography

// use secp256k1::{Secp256k1, SecretKey, PublicKey};
fn derive_public_key(private_key_hex: &str) -> String {
    // Step 1: Decode the private key from hex string to bytes

    // Step 2: Create a SecretKey object using secp256k1

    // Step 3: Derive the public key

    // Step 4: Convert the public key to its serialized format and encode as hex
    unimplemented!()
}

// MORE TASKS:
// https://github.com/berkingurcan/Patika-MerkleTree-Assignment
// https://github.com/zku-cohort-4/week4/tree/master/rust_assignment/freivald



fn main() {
    // You can print and test your functions here
    println!("Welcome to workshop 2")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_message() {
        // Check SHA256 hash for known input
        // "hello" -> SHA256: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
        let h = hash_message("hello");
        assert_eq!(h, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test_verify_hash() {
        let known_h = hash_message("secret");
        assert!(verify_hash("secret", &known_h));
        assert!(!verify_hash("not_secret", &known_h));
    }

    #[test]
    fn test_generate_nonce() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();
        assert!(nonce1 != nonce2 || nonce1 > 0);
    }

    #[test]
    fn test_generate_private_key() {
        let key_hex = generate_private_key();
        // 32 bytes in hex = 64 hex chars
        assert_eq!(key_hex.len(), 64);
    }

    #[test]
    fn test_derive_public_key() {
        let private_key_hex = generate_private_key();
        let public_key_hex = derive_public_key(&private_key_hex);

        assert!(!public_key_hex.is_empty());
        assert_eq!(public_key_hex.len(), 130);
    }
}


/*

  _____                _ _                 
 |  __ \              | (_)                
 | |__) |___  __ _  __| |_ _ __   __ _ ___ 
 |  _  // _ \/ _` |/ _` | | '_ \ / _` / __|
 | | \ \  __/ (_| | (_| | | | | | (_| \__ \
 |_|  \_\___|\__,_|\__,_|_|_| |_|\__, |___/
                                  __/ |    
                                 |___/     
        
               
https://www.activism.net/cypherpunk/manifesto.html                               
https://activism.net/cypherpunk/crypto-anarchy.html       
https://vitalik.eth.limo/general/2023/12/28/cypherpunk.html
https://youtu.be/J_Pov8cO7O4?si=AqtRm3EZg2b6v_5Z                         
*/