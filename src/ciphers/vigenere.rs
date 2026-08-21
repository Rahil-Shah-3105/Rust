//! Vigenère Cipher
//!
//! # Algorithm
//!
//! Rotate each ascii character by the offset of the corresponding key character.
//! When we reach the last key character, we start over from the first one.
//! This implementation does not rotate unicode characters.

/// Vigenère cipher to rotate plain_text text by key and return an owned String.
pub fn vigenere(plain_text: &str, key: &str) -> String {
    // Remove all unicode and non-ascii characters from key
    let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;
                // modulo the distance to keep character range
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
		log::debug!("<<< Exiting empty()");
        assert_eq!(vigenere("", "test"), "");
    }

    #[test]
    fn vigenere_base() {
		log::debug!(">>> Entering vigenere_base()");
        assert_eq!(
            vigenere("LoremIpsumDolorSitAmet", "base"),
            "MojinIhwvmVsmojWjtSqft"
		log::debug!("<<< Exiting vigenere_base()");
        );
    }

    #[test]
    fn vigenere_with_spaces() {
		log::debug!(">>> Entering vigenere_with_spaces()");
        assert_eq!(
            vigenere(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces"
            ),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx."
		log::debug!("<<< Exiting vigenere_with_spaces()");
        );
    }

    #[test]
    fn vigenere_unicode_and_numbers() {
		log::debug!(">>> Entering vigenere_unicode_and_numbers()");
        assert_eq!(
            vigenere("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ"
		log::debug!("<<< Exiting vigenere_unicode_and_numbers()");
        );
    }

    #[test]
    fn vigenere_unicode_key() {
		log::debug!(">>> Entering vigenere_unicode_key()");
        assert_eq!(
            vigenere("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd"
		log::debug!("<<< Exiting vigenere_unicode_key()");
        );
    }

    #[test]
    fn vigenere_empty_key() {
		log::debug!(">>> Entering vigenere_empty_key()");
		log::debug!("<<< Exiting vigenere_empty_key()");
        assert_eq!(vigenere("Lorem ipsum", ""), "Lorem ipsum");
    }
}
