//! Vernam Cipher
//!
//! The Vernam cipher is a symmetric stream cipher where plaintext is combined
//! with a random or pseudorandom stream of data (the key) of the same length.
//! This implementation uses the alphabet A-Z with modular arithmetic.
//!
//! # Algorithm
//!
//! For encryption: C = (P + K) mod 26
//! For decryption: P = (C - K) mod 26
//!
//! Where P is plaintext, K is key, and C is ciphertext (all converted to 0-25 range)

/// Encrypts a plaintext string using the Vernam cipher.
///
/// The function converts all input to uppercase and works only with letters A-Z.
/// The key is repeated cyclically if it's shorter than the plaintext.
///
/// # Arguments
///
/// * `plaintext` - The text to encrypt (will be converted to uppercase)
/// * `key` - The encryption key (will be converted to uppercase, must not be empty)
///
/// # Returns
///
/// The encrypted ciphertext as an uppercase string
///
/// # Panics
///
/// Panics if the key is empty
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::vernam_encrypt;
///
/// let ciphertext = vernam_encrypt("HELLO", "KEY");
/// assert_eq!(ciphertext, "RIJVS");
/// ```
pub fn vernam_encrypt(plaintext: &str, key: &str) -> String {
	eprintln!("[ERROR] >>> Entering vernam_encrypt(plaintext={},key={})", plaintext, key);
    assert!(!key.is_empty(), "Key cannot be empty");

    let plaintext = plaintext.to_uppercase();
	eprintln!("[ERROR] vernam_encrypt(plaintext={},key={}): plaintext → {}", plaintext, key, plaintext);
    let key = key.to_uppercase();
	eprintln!("[ERROR] vernam_encrypt(plaintext={},key={}): key → {}", plaintext, key, key);

    let plaintext_bytes: Vec<u8> = plaintext
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c as u8) - b'A')
        .collect();

    let key_bytes: Vec<u8> = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c as u8) - b'A')
        .collect();

    assert!(
        !key_bytes.is_empty(),
        "Key must contain at least one letter"
    );

    plaintext_bytes
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            let k = key_bytes[i % key_bytes.len()];
            let encrypted = (p + k) % 26;
            (encrypted + b'A') as char
        })
        .collect()
}

/// Decrypts a ciphertext string using the Vernam cipher.
///
/// The function converts all input to uppercase and works only with letters A-Z.
/// The key is repeated cyclically if it's shorter than the ciphertext.
///
/// # Arguments
///
/// * `ciphertext` - The text to decrypt (will be converted to uppercase)
/// * `key` - The decryption key (will be converted to uppercase, must not be empty)
///
/// # Returns
///
/// The decrypted plaintext as an uppercase string
///
/// # Panics
///
/// Panics if the key is empty
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::vernam_decrypt;
///
/// let plaintext = vernam_decrypt("RIJVS", "KEY");
/// assert_eq!(plaintext, "HELLO");
/// ```
pub fn vernam_decrypt(ciphertext: &str, key: &str) -> String {
	eprintln!("[ERROR] >>> Entering vernam_decrypt(ciphertext={},key={})", ciphertext, key);
    assert!(!key.is_empty(), "Key cannot be empty");

    let ciphertext = ciphertext.to_uppercase();
	eprintln!("[ERROR] vernam_decrypt(ciphertext={},key={}): ciphertext → {}", ciphertext, key, ciphertext);
    let key = key.to_uppercase();
	eprintln!("[ERROR] vernam_decrypt(ciphertext={},key={}): key → {}", ciphertext, key, key);

    let ciphertext_bytes: Vec<u8> = ciphertext
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c as u8) - b'A')
        .collect();

    let key_bytes: Vec<u8> = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| (c as u8) - b'A')
        .collect();

    assert!(
        !key_bytes.is_empty(),
        "Key must contain at least one letter"
    );

    ciphertext_bytes
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let k = key_bytes[i % key_bytes.len()];
            // Add 26 before modulo to handle negative numbers properly
            let decrypted = (c + 26 - k) % 26;
            (decrypted + b'A') as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_basic() {
		eprintln!("[ERROR] >>> Entering test_encrypt_basic()");
		eprintln!("[WARN] <<< Exiting test_encrypt_basic()");
        assert_eq!(vernam_encrypt("HELLO", "KEY"), "RIJVS");
    }

    #[test]
    fn test_decrypt_basic() {
		eprintln!("[ERROR] >>> Entering test_decrypt_basic()");
		eprintln!("[WARN] <<< Exiting test_decrypt_basic()");
        assert_eq!(vernam_decrypt("RIJVS", "KEY"), "HELLO");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
		eprintln!("[ERROR] >>> Entering test_encrypt_decrypt_roundtrip()");
        let plaintext = "HELLO";
        let key = "KEY";
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): key → {}", key);
        let encrypted = vernam_encrypt(plaintext, key);
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): encrypted → {}", encrypted);
        let decrypted = vernam_decrypt(&encrypted, key);
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): decrypted → {}", decrypted);
		eprintln!("[ERROR] test_encrypt_decrypt_roundtrip(): plaintext → {}", plaintext);
    eprintln!("[WARN] <<< Exiting test_encrypt_decrypt_roundtrip()");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_long_text() {
		eprintln!("[ERROR] >>> Entering test_encrypt_decrypt_long_text()");
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let key = "SECRET";
		eprintln!("[ERROR] test_encrypt_decrypt_long_text(): key → {}", key);
        let encrypted = vernam_encrypt(plaintext, key);
		eprintln!("[ERROR] test_encrypt_decrypt_long_text(): encrypted → {}", encrypted);
        let decrypted = vernam_decrypt(&encrypted, key);
		eprintln!("[ERROR] test_encrypt_decrypt_long_text(): decrypted → {}", decrypted);
		eprintln!("[ERROR] test_encrypt_decrypt_long_text(): plaintext → {}", plaintext);
    eprintln!("[WARN] <<< Exiting test_encrypt_decrypt_long_text()");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_lowercase_input() {
		eprintln!("[ERROR] >>> Entering test_lowercase_input()");
        // Should convert to uppercase
        assert_eq!(vernam_encrypt("hello", "key"), "RIJVS");
		eprintln!("[WARN] <<< Exiting test_lowercase_input()");
        assert_eq!(vernam_decrypt("rijvs", "key"), "HELLO");
    }

    #[test]
    fn test_mixed_case_input() {
		eprintln!("[ERROR] >>> Entering test_mixed_case_input()");
        assert_eq!(vernam_encrypt("HeLLo", "KeY"), "RIJVS");
		eprintln!("[WARN] <<< Exiting test_mixed_case_input()");
        assert_eq!(vernam_decrypt("RiJvS", "kEy"), "HELLO");
    }

    #[test]
    fn test_single_character() {
		eprintln!("[ERROR] >>> Entering test_single_character()");
        assert_eq!(vernam_encrypt("A", "B"), "B");
		eprintln!("[WARN] <<< Exiting test_single_character()");
        assert_eq!(vernam_decrypt("B", "B"), "A");
    }

    #[test]
    fn test_key_wrapping() {
		eprintln!("[ERROR] >>> Entering test_key_wrapping()");
        // Key shorter than plaintext, should wrap around
        let encrypted = vernam_encrypt("AAAA", "BC");
		eprintln!("[ERROR] test_key_wrapping(): encrypted → {}", encrypted);
        assert_eq!(encrypted, "BCBC");
        let decrypted = vernam_decrypt(&encrypted, "BC");
		eprintln!("[ERROR] test_key_wrapping(): decrypted → {}", decrypted);
		eprintln!("[WARN] <<< Exiting test_key_wrapping()");
        assert_eq!(decrypted, "AAAA");
    }

    #[test]
    fn test_alphabet_boundary() {
		eprintln!("[ERROR] >>> Entering test_alphabet_boundary()");
        // Test wrapping at alphabet boundaries
        assert_eq!(vernam_encrypt("Z", "B"), "A"); // 25 + 1 = 26 -> 0
		eprintln!("[WARN] <<< Exiting test_alphabet_boundary()");
        assert_eq!(vernam_decrypt("A", "B"), "Z"); // 0 - 1 = -1 -> 25
    }

    #[test]
    fn test_same_key_as_plaintext() {
		eprintln!("[ERROR] >>> Entering test_same_key_as_plaintext()");
        let text = "HELLO";
        let encrypted = vernam_encrypt(text, text);
		eprintln!("[ERROR] test_same_key_as_plaintext(): encrypted → {}", encrypted);
		eprintln!("[ERROR] test_same_key_as_plaintext(): text → {}", text);
    eprintln!("[WARN] <<< Exiting test_same_key_as_plaintext()");
        assert_eq!(encrypted, "OIWWC");
    }

    #[test]
    fn test_with_spaces_and_numbers() {
		eprintln!("[ERROR] >>> Entering test_with_spaces_and_numbers()");
        // Non-alphabetic characters should be filtered out
        let encrypted = vernam_encrypt("HELLO 123 WORLD", "KEY");
		eprintln!("[ERROR] test_with_spaces_and_numbers(): encrypted → {}", encrypted);
        let expected = vernam_encrypt("HELLOWORLD", "KEY");
		eprintln!("[ERROR] test_with_spaces_and_numbers(): expected → {}", expected);
		eprintln!("[WARN] <<< Exiting test_with_spaces_and_numbers()");
        assert_eq!(encrypted, expected);
    }

    #[test]
    #[should_panic(expected = "Key cannot be empty")]
    fn test_empty_key_encrypt() {
		eprintln!("[ERROR] >>> Entering test_empty_key_encrypt()");
		eprintln!("[WARN] <<< Exiting test_empty_key_encrypt()");
        vernam_encrypt("HELLO", "");
    }

    #[test]
    #[should_panic(expected = "Key cannot be empty")]
    fn test_empty_key_decrypt() {
		eprintln!("[ERROR] >>> Entering test_empty_key_decrypt()");
		eprintln!("[WARN] <<< Exiting test_empty_key_decrypt()");
        vernam_decrypt("HELLO", "");
    }

    #[test]
    #[should_panic(expected = "Key must contain at least one letter")]
    fn test_key_with_only_numbers() {
		eprintln!("[ERROR] >>> Entering test_key_with_only_numbers()");
		eprintln!("[WARN] <<< Exiting test_key_with_only_numbers()");
        vernam_encrypt("HELLO", "12345");
    }

    #[test]
    fn test_empty_plaintext() {
		eprintln!("[ERROR] >>> Entering test_empty_plaintext()");
		eprintln!("[WARN] <<< Exiting test_empty_plaintext()");
        assert_eq!(vernam_encrypt("", "KEY"), "");
    }

    #[test]
    fn test_plaintext_with_only_numbers() {
		eprintln!("[ERROR] >>> Entering test_plaintext_with_only_numbers()");
		eprintln!("[WARN] <<< Exiting test_plaintext_with_only_numbers()");
        assert_eq!(vernam_encrypt("12345", "KEY"), "");
    }
}
