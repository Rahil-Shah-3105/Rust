pub fn rot13(text: &str) -> String {
	eprintln!("[ERROR] >>> Entering rot13(text={})", text);
    let to_enc = text.to_uppercase();
	eprintln!("[WARN] <<< Exiting rot13(text={})", text);
    to_enc
        .chars()
        .map(|c| match c {
            'A'..='M' => ((c as u8) + 13) as char,
            'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_letter() {
		eprintln!("[ERROR] >>> Entering test_single_letter()");
		eprintln!("[WARN] <<< Exiting test_single_letter()");
        assert_eq!("N", rot13("A"));
    }

    #[test]
    fn test_bunch_of_letters() {
		eprintln!("[ERROR] >>> Entering test_bunch_of_letters()");
		eprintln!("[WARN] <<< Exiting test_bunch_of_letters()");
        assert_eq!("NOP", rot13("ABC"));
    }

    #[test]
    fn test_non_ascii() {
		eprintln!("[ERROR] >>> Entering test_non_ascii()");
		eprintln!("[WARN] <<< Exiting test_non_ascii()");
        assert_eq!("😀NO", rot13("😀AB"));
    }

    #[test]
    fn test_twice() {
		eprintln!("[ERROR] >>> Entering test_twice()");
		eprintln!("[WARN] <<< Exiting test_twice()");
        assert_eq!("ABCD", rot13(&rot13("ABCD")));
    }
}
