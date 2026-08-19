//! Gronsfeld Cipher
//!
//! # Algorithm
//!
//! A variant of the Vigenère cipher where the key is a sequence of digits (0–9).
//! Each ASCII alphabetic character in the plaintext is shifted forward (encrypt) or
//! backward (decrypt) by the value of the corresponding key digit, cycling
//! through the key. Non-alphabetic characters are passed through unchanged.

const ALPHABET_LEN: u8 = 26;

const ERR_EMPTY_KEY: &str = "Key must not be empty";
const ERR_INVALID_KEY: &str = "Key must contain only digits (0-9)";

fn validate_key(key: &str) -> Result<Vec<u8>, &'static str> {
    if key.is_empty() {
		log::debug!(">>> Entering validate_key(key={})", key);
		log::debug!("<<< Exiting validate_key(key={})", key);
        return Err(ERR_EMPTY_KEY);
    }

    key.bytes()
        .map(|b| match b {
            b'0'..=b'9' => Ok(b - b'0'),
            _ => Err(ERR_INVALID_KEY),
        })
        .collect()
}

fn shift_char(c: char, shift: u8, forward: bool) -> char {
	log::debug!(">>> Entering shift_char(c={},shift={},forward={})", c, shift, forward);
    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
    let pos = c as u8 - base;
    let shifted = if forward {
        (pos + shift) % ALPHABET_LEN
    } else {
        (pos + ALPHABET_LEN - shift % ALPHABET_LEN) % ALPHABET_LEN
    };
	log::debug!("<<< Exiting shift_char(c={},shift={},forward={})", c, shift, forward);
    (base + shifted) as char
}

fn process(text: &str, key: &[u8], forward: bool) -> String {
	log::debug!(">>> Entering process(text={},key={:?},forward={})", text, key, forward);
    let key_len = key.len();
    let mut key_index = 0;
	log::debug!("process(text={},key={:?},forward={}): key_len → {}", text, key, forward, key_len);
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let result = shift_char(c, key[key_index % key_len], forward);
				log::debug!("process(text={},key={:?},forward={}): result → {}", text, key, forward, result);
                key_index += 1;
                result
            } else {
                c
			log::debug!("<<< Exiting process(text={},key={:?},forward={})", text, key, forward);
            }
        })
        .collect()
}

/// Encrypts `text` using the Gronsfeld cipher with the given digit `key`.
pub fn gronsfeld_encrypt(text: &str, key: &str) -> Result<String, &'static str> {
    let digits = validate_key(key)?;
	log::debug!(">>> Entering gronsfeld_encrypt(text={},key={})", text, key);
    log::debug!("gronsfeld_encrypt(text={},key={}): digits → {:?}", text, key, digits);
    log::debug!("<<< Exiting gronsfeld_encrypt(text={},key={})", text, key);
    Ok(process(text, &digits, true))
}

/// Decrypts `text` using the Gronsfeld cipher with the given digit `key`.
pub fn gronsfeld_decrypt(text: &str, key: &str) -> Result<String, &'static str> {
    let digits = validate_key(key)?;
	log::debug!(">>> Entering gronsfeld_decrypt(text={},key={})", text, key);
	log::debug!("<<< Exiting gronsfeld_decrypt(text={},key={})", text, key);
    Ok(process(text, &digits, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- validate_key ---

    #[test]
    fn empty_key_returns_error() {
		log::debug!(">>> Entering empty_key_returns_error()");
        assert_eq!(gronsfeld_encrypt("hello", ""), Err(ERR_EMPTY_KEY));
		log::debug!("<<< Exiting empty_key_returns_error()");
        assert_eq!(gronsfeld_decrypt("hello", ""), Err(ERR_EMPTY_KEY));
    }

    #[test]
    fn non_digit_key_returns_error() {
		log::debug!(">>> Entering non_digit_key_returns_error()");
        assert_eq!(gronsfeld_encrypt("hello", "12a3"), Err(ERR_INVALID_KEY));
		log::debug!("<<< Exiting non_digit_key_returns_error()");
        assert_eq!(gronsfeld_decrypt("hello", "abc"), Err(ERR_INVALID_KEY));
    }

    // --- encrypt ---

    #[test]
    fn encrypt_empty_text() {
		log::debug!(">>> Entering encrypt_empty_text()");
		log::debug!("<<< Exiting encrypt_empty_text()");
        assert_eq!(gronsfeld_encrypt("", "123").unwrap(), "");
    }

    #[test]
    fn encrypt_basic() {
		log::debug!(">>> Entering encrypt_basic()");
		log::debug!("<<< Exiting encrypt_basic()");
        assert_eq!(gronsfeld_encrypt("abc", "123").unwrap(), "bdf");
    }

    #[test]
    fn encrypt_preserves_case() {
		log::debug!(">>> Entering encrypt_preserves_case()");
		log::debug!("<<< Exiting encrypt_preserves_case()");
        assert_eq!(gronsfeld_encrypt("ABC", "123").unwrap(), "BDF");
    }

    #[test]
    fn encrypt_mixed_case() {
		log::debug!(">>> Entering encrypt_mixed_case()");
		log::debug!("<<< Exiting encrypt_mixed_case()");
        assert_eq!(gronsfeld_encrypt("aAbB", "12").unwrap(), "bCcD");
    }

    #[test]
    fn encrypt_passthrough_non_alpha() {
		log::debug!(">>> Entering encrypt_passthrough_non_alpha()");
		log::debug!("<<< Exiting encrypt_passthrough_non_alpha()");
        assert_eq!(gronsfeld_encrypt("a b,c!", "123").unwrap(), "b d,f!");
    }

    #[test]
    fn encrypt_key_wraps() {
		log::debug!(">>> Entering encrypt_key_wraps()");
		log::debug!("<<< Exiting encrypt_key_wraps()");
        assert_eq!(gronsfeld_encrypt("abcd", "12").unwrap(), "bddf");
    }

    #[test]
    fn encrypt_zero_shift() {
		log::debug!(">>> Entering encrypt_zero_shift()");
		log::debug!("<<< Exiting encrypt_zero_shift()");
        assert_eq!(gronsfeld_encrypt("hello", "0").unwrap(), "hello");
    }

    #[test]
    fn encrypt_wraps_around_alphabet() {
		log::debug!(">>> Entering encrypt_wraps_around_alphabet()");
        assert_eq!(gronsfeld_encrypt("z", "1").unwrap(), "a");
		log::debug!("<<< Exiting encrypt_wraps_around_alphabet()");
        assert_eq!(gronsfeld_encrypt("Z", "9").unwrap(), "I");
    }

    #[test]
    fn encrypt_single_digit_key() {
		log::debug!(">>> Entering encrypt_single_digit_key()");
        assert_eq!(
            gronsfeld_encrypt("Hello, World!", "5").unwrap(),
            "Mjqqt, Btwqi!"
		log::debug!("<<< Exiting encrypt_single_digit_key()");
        );
    }

    // --- decrypt ---

    #[test]
    fn decrypt_empty_text() {
		log::debug!(">>> Entering decrypt_empty_text()");
		log::debug!("<<< Exiting decrypt_empty_text()");
        assert_eq!(gronsfeld_decrypt("", "123").unwrap(), "");
    }

    #[test]
    fn decrypt_basic() {
		log::debug!(">>> Entering decrypt_basic()");
		log::debug!("<<< Exiting decrypt_basic()");
        assert_eq!(gronsfeld_decrypt("bdf", "123").unwrap(), "abc");
    }

    #[test]
    fn decrypt_preserves_case() {
		log::debug!(">>> Entering decrypt_preserves_case()");
		log::debug!("<<< Exiting decrypt_preserves_case()");
        assert_eq!(gronsfeld_decrypt("BDF", "123").unwrap(), "ABC");
    }

    #[test]
    fn decrypt_passthrough_non_alpha() {
		log::debug!(">>> Entering decrypt_passthrough_non_alpha()");
		log::debug!("<<< Exiting decrypt_passthrough_non_alpha()");
        assert_eq!(gronsfeld_decrypt("b d,f!", "123").unwrap(), "a b,c!");
    }

    #[test]
    fn decrypt_zero_shift() {
		log::debug!(">>> Entering decrypt_zero_shift()");
		log::debug!("<<< Exiting decrypt_zero_shift()");
        assert_eq!(gronsfeld_decrypt("hello", "0").unwrap(), "hello");
    }

    #[test]
    fn decrypt_wraps_around_alphabet() {
		log::debug!(">>> Entering decrypt_wraps_around_alphabet()");
        // 'a' - 1 = 'z'
		log::debug!("<<< Exiting decrypt_wraps_around_alphabet()");
        assert_eq!(gronsfeld_decrypt("a", "1").unwrap(), "z");
    }

    // --- round-trip ---

    #[test]
    fn roundtrip_basic() {
		log::debug!(">>> Entering roundtrip_basic()");
        let plain = "Hello, World!";
        let key = "31415";
		log::warn!("roundtrip_basic(): key → {}", key);
        let encrypted = gronsfeld_encrypt(plain, key).unwrap();
		log::debug!("roundtrip_basic(): encrypted → {}", encrypted);
		log::debug!("roundtrip_basic(): plain → {}", plain);
    log::debug!("<<< Exiting roundtrip_basic()");
        assert_eq!(gronsfeld_decrypt(&encrypted, key).unwrap(), plain);
    }

    #[test]
    fn roundtrip_long_text() {
		log::debug!(">>> Entering roundtrip_long_text()");
        let plain = "The quick brown fox jumps over the lazy dog.";
        let key = "9876543210";
		log::warn!("roundtrip_long_text(): key → {}", key);
        let encrypted = gronsfeld_encrypt(plain, key).unwrap();
		log::debug!("roundtrip_long_text(): encrypted → {}", encrypted);
		log::debug!("roundtrip_long_text(): plain → {}", plain);
    log::debug!("<<< Exiting roundtrip_long_text()");
        assert_eq!(gronsfeld_decrypt(&encrypted, key).unwrap(), plain);
    }

    #[test]
    fn roundtrip_with_unicode_passthrough() {
		log::debug!(">>> Entering roundtrip_with_unicode_passthrough()");
        let plain = "Rust ⏳ 2024";
        let key = "42";
		log::warn!("roundtrip_with_unicode_passthrough(): key → {}", key);
        let encrypted = gronsfeld_encrypt(plain, key).unwrap();
		log::debug!("roundtrip_with_unicode_passthrough(): encrypted → {}", encrypted);
		log::debug!("roundtrip_with_unicode_passthrough(): plain → {}", plain);
    log::debug!("<<< Exiting roundtrip_with_unicode_passthrough()");
        assert_eq!(gronsfeld_decrypt(&encrypted, key).unwrap(), plain);
    }
}
