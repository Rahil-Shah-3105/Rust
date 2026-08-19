use std::collections::HashMap;
use std::io;

const UNKNOWN_CHARACTER: &str = "........";
const _UNKNOWN_MORSE_CHARACTER: &str = "_";

pub fn encode(message: &str) -> String {
	log::debug!(">>> Entering encode(message={})", message);
    let dictionary = _morse_dictionary();
	log::debug!("encode(message={}): dictionary → {:?}", message, dictionary);
    log::debug!("<<< Exiting encode(message={})", message);
    message
        .chars()
        .map(|char| char.to_uppercase().to_string())
        .map(|letter| dictionary.get(letter.as_str()))
        .map(|option| (*option.unwrap_or(&UNKNOWN_CHARACTER)).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// Declarative macro for creating readable map declarations, for more info see https://doc.rust-lang.org/book/ch19-06-macros.html
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([$(($key, $value),)*]))
    };
}

fn _morse_dictionary() -> HashMap<&'static str, &'static str> {
	log::debug!(">>> Entering _morse_dictionary()");
    map! {
        "A" => ".-",      "B" => "-...",    "C" => "-.-.",
        "D" => "-..",     "E" => ".",       "F" => "..-.",
        "G" => "--.",     "H" => "....",    "I" => "..",
        "J" => ".---",    "K" => "-.-",     "L" => ".-..",
        "M" => "--",      "N" => "-.",      "O" => "---",
        "P" => ".--.",    "Q" => "--.-",    "R" => ".-.",
        "S" => "...",     "T" => "-",       "U" => "..-",
        "V" => "...-",    "W" => ".--",     "X" => "-..-",
        "Y" => "-.--",    "Z" => "--..",

        "1" => ".----",   "2" => "..---",   "3" => "...--",
        "4" => "....-",   "5" => ".....",   "6" => "-....",
        "7" => "--...",   "8" => "---..",   "9" => "----.",
        "0" => "-----",

        "&" => ".-...",   "@" => ".--.-.",  ":" => "---...",
        "," => "--..--",  "." => ".-.-.-",  "'" => ".----.",
        "\"" => ".-..-.", "?" => "..--..",  "/" => "-..-.",
        "=" => "-...-",   "+" => ".-.-.",   "-" => "-....-",
        "(" => "-.--.",   ")" => "-.--.-",  " " => "/",
        "!" => "-.-.--",
    }
}

fn _morse_to_alphanumeric_dictionary() -> HashMap<&'static str, &'static str> {
	log::debug!(">>> Entering _morse_to_alphanumeric_dictionary()");
    map! {
        ".-"   =>  "A",      "-..." => "B",    "-.-." => "C",
        "-.."  =>  "D",      "."    => "E",       "..-." => "F",
        "--."  =>  "G",      "...." => "H",    ".." => "I",
        ".---" =>  "J",     "-.-" => "K",     ".-.." => "L",
        "--"   =>  "M",       "-." => "N",      "---" => "O",
        ".--." =>  "P",     "--.-" => "Q",    ".-." => "R",
        "..."  =>  "S",      "-" => "T",       "..-" => "U",
        "...-" =>  "V",     ".--" => "W",     "-..-" => "X",
        "-.--" =>  "Y",     "--.." => "Z",

        ".----" => "1",    "..---" => "2",   "...--" => "3",
        "....-" => "4",    "....." => "5",   "-...." => "6",
        "--..." => "7",    "---.." => "8",   "----." => "9",
        "-----" => "0",

        ".-..." => "&",    ".--.-." => "@",  "---..." => ":",
        "--..--" => ",",   ".-.-.-" => ".",  ".----." => "'",
        ".-..-." => "\"",  "..--.." => "?",  "-..-." => "/",
        "-...-" => "=",   ".-.-." => "+",   "-....-" => "-",
        "-.--." => "(",   "-.--.-" => ")",  "/" => " ",
        "-.-.--" => "!",  " " => " ",       "" => ""
    }
}

fn _check_part(string: &str) -> bool {
	log::debug!(">>> Entering _check_part(string={})", string);
    for c in string.chars() {
        match c {
            '.' | '-' | ' ' => (),
            _ => return false,
        }
    }
	log::debug!("<<< Exiting _check_part(string={})", string);
    true
}

fn _check_all_parts(string: &str) -> bool {
	log::debug!(">>> Entering _check_all_parts(string={})", string);
    string.split('/').all(_check_part)
}

fn _decode_token(string: &str) -> String {
	log::debug!(">>> Entering _decode_token(string={})", string);
    (*_morse_to_alphanumeric_dictionary()
        .get(string)
        .unwrap_or(&_UNKNOWN_MORSE_CHARACTER))
    .to_string()
}

fn _decode_part(string: &str) -> String {
	log::debug!(">>> Entering _decode_part(string={})", string);
    string.split(' ').map(_decode_token).collect::<String>()
}

/// Convert morse code to ascii.
///
/// Given a morse code, return the corresponding message.
/// If the code is invalid, the undecipherable part of the code is replaced by `_`.
pub fn decode(string: &str) -> Result<String, io::Error> {
	log::debug!(">>> Entering decode(string={})", string);
    if !_check_all_parts(string) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid morse code",
        ));
    }

    let mut partitions: Vec<String> = vec![];

    for part in string.split('/') {
        partitions.push(_decode_part(part));
    }

    Ok(partitions.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_only_letters() {
		log::debug!(">>> Entering encrypt_only_letters()");
        let message = "Hello Morse";
        let cipher = encode(message);
		log::debug!("encrypt_only_letters(): cipher → {}", cipher);
        log::debug!("encrypt_only_letters(): message → {}", message);
        log::debug!("<<< Exiting encrypt_only_letters()");
        assert_eq!(
            cipher,
            ".... . .-.. .-.. --- / -- --- .-. ... .".to_string()
        )
    }

    #[test]
    fn encrypt_letters_and_special_characters() {
		log::debug!(">>> Entering encrypt_letters_and_special_characters()");
        let message = "What's a great day!";
        let cipher = encode(message);
		log::debug!("encrypt_letters_and_special_characters(): cipher → {}", cipher);
        log::debug!("encrypt_letters_and_special_characters(): message → {}", message);
        log::debug!("<<< Exiting encrypt_letters_and_special_characters()");
        assert_eq!(
            cipher,
            ".-- .... .- - .----. ... / .- / --. .-. . .- - / -.. .- -.-- -.-.--".to_string()
        )
    }

    #[test]
    fn encrypt_message_with_unsupported_character() {
		log::debug!(">>> Entering encrypt_message_with_unsupported_character()");
        let message = "Error?? {}";
        let cipher = encode(message);
		log::debug!("encrypt_message_with_unsupported_character(): cipher → {}", cipher);
        log::debug!("encrypt_message_with_unsupported_character(): message → {}", message);
        log::debug!("<<< Exiting encrypt_message_with_unsupported_character()");
        assert_eq!(
            cipher,
            ". .-. .-. --- .-. ..--.. ..--.. / ........ ........".to_string()
        )
    }

    #[test]
    fn decrypt_valid_morsecode_with_spaces() {
		log::debug!(">>> Entering decrypt_valid_morsecode_with_spaces()");
        let expected = "Hello Morse! How's it goin, \"eh\"?"
            .to_string()
            .to_uppercase();
        let encypted = encode(&expected);
		log::debug!("decrypt_valid_morsecode_with_spaces(): encypted → {}", encypted);
        let result = decode(&encypted).unwrap();
		log::debug!("decrypt_valid_morsecode_with_spaces(): result → {}", result);

		log::debug!("<<< Exiting decrypt_valid_morsecode_with_spaces()");
        assert_eq!(expected, result);
    }

    #[test]
    fn decrypt_valid_character_set_invalid_morsecode() {
		log::debug!(">>> Entering decrypt_valid_character_set_invalid_morsecode()");
        let expected = format!(
            "{_UNKNOWN_MORSE_CHARACTER}{_UNKNOWN_MORSE_CHARACTER}{_UNKNOWN_MORSE_CHARACTER}{_UNKNOWN_MORSE_CHARACTER} {_UNKNOWN_MORSE_CHARACTER}",
        );

        let encypted = ".-.-.--.-.-. --------. ..---.-.-. .-.-.--.-.-. / .-.-.--.-.-.".to_string();
		log::debug!("decrypt_valid_character_set_invalid_morsecode(): encypted → {}", encypted);
        let result = decode(&encypted).unwrap();
		log::debug!("decrypt_valid_character_set_invalid_morsecode(): result → {}", result);

		log::debug!("<<< Exiting decrypt_valid_character_set_invalid_morsecode()");
        assert_eq!(expected, result);
    }

    #[test]
    fn decrypt_invalid_morsecode_with_spaces() {
		log::debug!(">>> Entering decrypt_invalid_morsecode_with_spaces()");
        let encypted = "1... . .-.. .-.. --- / -- --- .-. ... .";
        let result = decode(encypted).map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::InvalidData);

		log::debug!("decrypt_invalid_morsecode_with_spaces(): encypted → {}", encypted);
		log::debug!("<<< Exiting decrypt_invalid_morsecode_with_spaces()");
        assert_eq!(expected, result);
    }
}
