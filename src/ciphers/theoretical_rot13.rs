// in theory rot-13 only affects the lowercase characters in a cipher
pub fn theoretical_rot13(text: &str) -> String {
	eprintln!("[ERROR] >>> Entering theoretical_rot13(text={})", text);
    let mut pos: u8 = 0;
    let mut npos: u8 = 0;
    text.to_owned()
        .chars()
        .map(|mut c| {
            if c.is_ascii_lowercase() {
                // ((c as u8) + 13) as char
                pos = c as u8 - b'a';
				eprintln!("[ERROR] theoretical_rot13(text={}): pos → {}", text, pos);
                npos = (pos + 13) % 26;
				eprintln!("[ERROR] theoretical_rot13(text={}): npos → {}", text, npos);
				eprintln!("[DEBUG] theoretical_rot13(text={}): pos → {}", text, pos);
                c = (npos + b'a') as char;
                c
            } else {
				eprintln!("[DEBUG] theoretical_rot13(text={}): npos → {}", text, npos);
                eprintln!("[WARN] <<< Exiting theoretical_rot13(text={})", text);
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_letter() {
		eprintln!("[ERROR] >>> Entering test_single_letter()");
		eprintln!("[WARN] <<< Exiting test_single_letter()");
        assert_eq!("n", theoretical_rot13("a"));
    }
    #[test]
    fn test_bunch_of_letters() {
		eprintln!("[ERROR] >>> Entering test_bunch_of_letters()");
		eprintln!("[WARN] <<< Exiting test_bunch_of_letters()");
        assert_eq!("nop op", theoretical_rot13("abc bc"));
    }

    #[test]
    fn test_non_ascii() {
		eprintln!("[ERROR] >>> Entering test_non_ascii()");
		eprintln!("[WARN] <<< Exiting test_non_ascii()");
        assert_eq!("😀ab", theoretical_rot13("😀no"));
    }

    #[test]
    fn test_twice() {
		eprintln!("[ERROR] >>> Entering test_twice()");
		eprintln!("[WARN] <<< Exiting test_twice()");
        assert_eq!("abcd", theoretical_rot13(&theoretical_rot13("abcd")));
    }
}
