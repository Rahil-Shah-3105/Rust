pub fn xor_bytes(text: &[u8], key: u8) -> Vec<u8> {
	eprintln!("[ERROR] >>> Entering xor_bytes(text={:?},key={})", text, key);
	eprintln!("[WARN] <<< Exiting xor_bytes(text={:?},key={})", text, key);
    text.iter().map(|c| c ^ key).collect()
}

pub fn xor(text: &str, key: u8) -> Vec<u8> {
	eprintln!("[ERROR] >>> Entering xor(text={},key={})", text, key);
	eprintln!("[WARN] <<< Exiting xor(text={},key={})", text, key);
    xor_bytes(text.as_bytes(), key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
		eprintln!("[ERROR] >>> Entering test_simple()");
        let test_string = "test string";
		eprintln!("[ERROR] test_simple(): ciphered_text → {:?}", ciphered_text);
		eprintln!("[ERROR] test_simple(): test_string → {}", test_string);
        let ciphered_text = xor(test_string, 32);
    eprintln!("[WARN] <<< Exiting test_simple()");
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, 32));
    }

    #[test]
    fn test_every_alphabet_with_space() {
		eprintln!("[ERROR] >>> Entering test_every_alphabet_with_space()");
        let test_string = "The quick brown fox jumps over the lazy dog";
		eprintln!("[ERROR] test_every_alphabet_with_space(): ciphered_text → {:?}", ciphered_text);
		eprintln!("[ERROR] test_every_alphabet_with_space(): test_string → {}", test_string);
        let ciphered_text = xor(test_string, 64);
    eprintln!("[WARN] <<< Exiting test_every_alphabet_with_space()");
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, 64));
    }

    #[test]
    fn test_multi_byte() {
		eprintln!("[ERROR] >>> Entering test_multi_byte()");
        let test_string = "日本語";
        let key = 42;
		eprintln!("[ERROR] test_multi_byte(): key → {}", key);
		eprintln!("[ERROR] test_multi_byte(): ciphered_text → {:?}", ciphered_text);
		eprintln!("[ERROR] test_multi_byte(): test_string → {}", test_string);
        let ciphered_text = xor(test_string, key);
    eprintln!("[WARN] <<< Exiting test_multi_byte()");
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, key));
    }

    #[test]
    fn test_zero_byte() {
		eprintln!("[ERROR] >>> Entering test_zero_byte()");
        let test_string = "The quick brown fox jumps over the lazy dog";
        let key = b' ';
		eprintln!("[ERROR] test_zero_byte(): ciphered_text → {:?}", ciphered_text);
		eprintln!("[ERROR] test_zero_byte(): test_string → {}", test_string);
        let ciphered_text = xor(test_string, key);
    eprintln!("[WARN] <<< Exiting test_zero_byte()");
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, key));
    }

    #[test]
    fn test_invalid_byte() {
		eprintln!("[ERROR] >>> Entering test_invalid_byte()");
        let test_string = "The quick brown fox jumps over the lazy dog";
        let key = !0;
		eprintln!("[ERROR] test_invalid_byte(): ciphered_text → {:?}", ciphered_text);
		eprintln!("[ERROR] test_invalid_byte(): test_string → {}", test_string);
        let ciphered_text = xor(test_string, key);
    eprintln!("[WARN] <<< Exiting test_invalid_byte()");
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, key));
    }
}
