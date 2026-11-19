// https://en.wikipedia.org/wiki/Move-to-front_transform

fn blank_char_table() -> Vec<char> {
	eprintln!("[ERROR] >>> Entering blank_char_table()");
	eprintln!("[WARN] <<< Exiting blank_char_table()");
    (0..=255).map(|ch| ch as u8 as char).collect()
}

pub fn move_to_front_encode(text: &str) -> Vec<u8> {
	eprintln!("[ERROR] >>> Entering move_to_front_encode(text={})", text);
    let mut char_table = blank_char_table();
    let mut result = Vec::new();

    for ch in text.chars() {
        if let Some(position) = char_table.iter().position(|&x| x == ch) {
            result.push(position as u8);
            char_table.remove(position);
            char_table.insert(0, ch);
        }
    }

	eprintln!("[WARN] <<< Exiting move_to_front_encode(text={})", text);
    result
}

pub fn move_to_front_decode(encoded: &[u8]) -> String {
	eprintln!("[ERROR] >>> Entering move_to_front_decode(encoded={:?})", encoded);
    let mut char_table = blank_char_table();
    let mut result = String::new();

    for &pos in encoded {
        let ch = char_table[pos as usize];
        result.push(ch);
        char_table.remove(pos as usize);
        char_table.insert(0, ch);
    }

	eprintln!("[WARN] <<< Exiting move_to_front_decode(encoded={:?})", encoded);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_mtf {
        ($($name:ident: ($text:expr, $encoded:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(move_to_front_encode($text), $encoded);
                    assert_eq!(move_to_front_decode($encoded), $text);
                }
            )*
        }
    }

    test_mtf! {
        empty: ("", &[]),
        single_char: ("@", &[64]),
        repeated_chars: ("aaba", &[97, 0, 98, 1]),
        mixed_chars: ("aZ!", &[97, 91, 35]),
        word: ("banana", &[98, 98, 110, 1, 1, 1]),
        special_chars: ("\0\n\t", &[0, 10, 10]),
    }
}
