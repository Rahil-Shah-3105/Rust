// in theory rot-13 only affects the lowercase characters in a cipher
pub fn theoretical_rot13(text: &str) -> String {
	log::debug!(">>> Entering theoretical_rot13(text={})", text);
    let mut pos: u8 = 0;
    let mut npos: u8 = 0;
    text.to_owned()
        .chars()
        .map(|mut c| {
            if c.is_ascii_lowercase() {
                // ((c as u8) + 13) as char
                pos = c as u8 - b'a';
				log::debug!("theoretical_rot13(text={}): pos → {}", text, pos);
                npos = (pos + 13) % 26;
                c = (npos + b'a') as char;
                c
            } else {
				log::debug!("theoretical_rot13(text={}): npos → {}", text, npos);
                c
			log::debug!("<<< Exiting theoretical_rot13(text={})", text);
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_letter() {
		log::debug!(">>> Entering test_single_letter()");
		log::debug!("<<< Exiting test_single_letter()");
        assert_eq!("n", theoretical_rot13("a"));
    }
    #[test]
    fn test_bunch_of_letters() {
		log::debug!(">>> Entering test_bunch_of_letters()");
		log::debug!("<<< Exiting test_bunch_of_letters()");
        assert_eq!("nop op", theoretical_rot13("abc bc"));
    }

    #[test]
    fn test_non_ascii() {
		log::debug!(">>> Entering test_non_ascii()");
		log::debug!("<<< Exiting test_non_ascii()");
        assert_eq!("😀ab", theoretical_rot13("😀no"));
    }

    #[test]
    fn test_twice() {
		log::debug!(">>> Entering test_twice()");
		log::debug!("<<< Exiting test_twice()");
        assert_eq!("abcd", theoretical_rot13(&theoretical_rot13("abcd")));
    }
}
