pub fn another_rot13(text: &str) -> String {
	log::debug!(">>> Entering another_rot13(text={})", text);
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let output = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm";
	log::debug!("another_rot13(text={}): output → {}", text, output);
    log::debug!("another_rot13(text={}): input → {}", text, input);
    log::debug!("<<< Exiting another_rot13(text={})", text);
    text.chars()
        .map(|c| match input.find(c) {
            Some(i) => output.chars().nth(i).unwrap(),
            None => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple() {
		log::debug!(">>> Entering test_simple()");
		log::debug!("<<< Exiting test_simple()");
        assert_eq!(another_rot13("ABCzyx"), "NOPmlk");
    }

    #[test]
    fn test_every_alphabet_with_space() {
		log::debug!(">>> Entering test_every_alphabet_with_space()");
        assert_eq!(
            another_rot13("The quick brown fox jumps over the lazy dog"),
            "Gur dhvpx oebja sbk whzcf bire gur ynml qbt"
		log::debug!("<<< Exiting test_every_alphabet_with_space()");
        );
    }

    #[test]
    fn test_non_alphabet() {
		log::debug!(">>> Entering test_non_alphabet()");
		log::debug!("<<< Exiting test_non_alphabet()");
        assert_eq!(another_rot13("🎃 Jack-o'-lantern"), "🎃 Wnpx-b'-ynagrea");
    }
}
