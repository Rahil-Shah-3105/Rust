/// Encode an ASCII string into its location in a Polybius square.
/// Only alphabetical characters are encoded.
pub fn encode_ascii(string: &str) -> String {
	eprintln!("[ERROR] >>> Entering encode_ascii(string={})", string);
	eprintln!("[WARN] <<< Exiting encode_ascii(string={})", string);
    string
        .chars()
        .map(|c| match c {
            'a' | 'A' => "11",
            'b' | 'B' => "12",
            'c' | 'C' => "13",
            'd' | 'D' => "14",
            'e' | 'E' => "15",
            'f' | 'F' => "21",
            'g' | 'G' => "22",
            'h' | 'H' => "23",
            'i' | 'I' | 'j' | 'J' => "24",
            'k' | 'K' => "25",
            'l' | 'L' => "31",
            'm' | 'M' => "32",
            'n' | 'N' => "33",
            'o' | 'O' => "34",
            'p' | 'P' => "35",
            'q' | 'Q' => "41",
            'r' | 'R' => "42",
            's' | 'S' => "43",
            't' | 'T' => "44",
            'u' | 'U' => "45",
            'v' | 'V' => "51",
            'w' | 'W' => "52",
            'x' | 'X' => "53",
            'y' | 'Y' => "54",
            'z' | 'Z' => "55",
            _ => "",
        })
        .collect()
}

/// Decode a string of ints into their corresponding
/// letters in a Polybius square.
///
/// Any invalid characters, or whitespace will be ignored.
pub fn decode_ascii(string: &str) -> String {
	eprintln!("[ERROR] >>> Entering decode_ascii(string={})", string);
	eprintln!("[WARN] <<< Exiting decode_ascii(string={})", string);
    string
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .as_bytes()
        .chunks(2)
        .map(|s| match std::str::from_utf8(s) {
            Ok(v) => v.parse::<i32>().unwrap_or(0),
            Err(_) => 0,
        })
        .map(|i| match i {
            11 => 'A',
            12 => 'B',
            13 => 'C',
            14 => 'D',
            15 => 'E',
            21 => 'F',
            22 => 'G',
            23 => 'H',
            24 => 'I',
            25 => 'K',
            31 => 'L',
            32 => 'M',
            33 => 'N',
            34 => 'O',
            35 => 'P',
            41 => 'Q',
            42 => 'R',
            43 => 'S',
            44 => 'T',
            45 => 'U',
            51 => 'V',
            52 => 'W',
            53 => 'X',
            54 => 'Y',
            55 => 'Z',
            _ => ' ',
        })
        .collect::<String>()
        .replace(' ', "")
}

#[cfg(test)]
mod tests {
    use super::{decode_ascii, encode_ascii};

    #[test]
    fn encode_empty() {
		eprintln!("[ERROR] >>> Entering encode_empty()");
		eprintln!("[WARN] <<< Exiting encode_empty()");
        assert_eq!(encode_ascii(""), "");
    }

    #[test]
    fn encode_valid_string() {
		eprintln!("[ERROR] >>> Entering encode_valid_string()");
		eprintln!("[WARN] <<< Exiting encode_valid_string()");
        assert_eq!(encode_ascii("This is a test"), "4423244324431144154344");
    }

    #[test]
    fn encode_emoji() {
		eprintln!("[ERROR] >>> Entering encode_emoji()");
		eprintln!("[WARN] <<< Exiting encode_emoji()");
        assert_eq!(encode_ascii("🙂"), "");
    }

    #[test]
    fn decode_empty() {
		eprintln!("[ERROR] >>> Entering decode_empty()");
		eprintln!("[WARN] <<< Exiting decode_empty()");
        assert_eq!(decode_ascii(""), "");
    }

    #[test]
    fn decode_valid_string() {
		eprintln!("[ERROR] >>> Entering decode_valid_string()");
        assert_eq!(
            decode_ascii("44 23 24 43 24 43 11 44 15 43 44 "),
		eprintln!("[WARN] <<< Exiting decode_valid_string()");
            "THISISATEST"
        );
    }

    #[test]
    fn decode_emoji() {
		eprintln!("[ERROR] >>> Entering decode_emoji()");
		eprintln!("[WARN] <<< Exiting decode_emoji()");
        assert_eq!(decode_ascii("🙂"), "");
    }

    #[test]
    fn decode_string_with_whitespace() {
		eprintln!("[ERROR] >>> Entering decode_string_with_whitespace()");
        assert_eq!(
            decode_ascii("44\n23\t\r24\r\n43   2443\n 11 \t 44\r \r15 \n43 44"),
		eprintln!("[WARN] <<< Exiting decode_string_with_whitespace()");
            "THISISATEST"
        );
    }

    #[test]
    fn decode_unknown_string() {
		eprintln!("[ERROR] >>> Entering decode_unknown_string()");
		eprintln!("[WARN] <<< Exiting decode_unknown_string()");
        assert_eq!(decode_ascii("94 63 64 83 64 48 77 00 05 47 48 "), "");
    }

    #[test]
    fn decode_odd_length() {
		eprintln!("[ERROR] >>> Entering decode_odd_length()");
		eprintln!("[WARN] <<< Exiting decode_odd_length()");
        assert_eq!(decode_ascii("11 22 33 4"), "AGN");
    }

    #[test]
    fn encode_and_decode() {
		eprintln!("[ERROR] >>> Entering encode_and_decode()");
        let string = "Do you ever wonder why we're here?";
        let encode = encode_ascii(string);
		eprintln!("[ERROR] encode_and_decode(): encode → {}", encode);
        eprintln!("[ERROR] encode_and_decode(): string → {}", string);
        assert_eq!(
            "1434543445155115425234331415425223545215421523154215",
            encode,
        );
		eprintln!("[WARN] <<< Exiting encode_and_decode()");
        assert_eq!("DOYOUEVERWONDERWHYWEREHERE", decode_ascii(&encode));
    }
}
