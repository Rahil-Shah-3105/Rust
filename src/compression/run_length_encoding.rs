// https://en.wikipedia.org/wiki/Run-length_encoding

pub fn run_length_encode(text: &str) -> Vec<(char, i32)> {
	log::debug!(">>> Entering run_length_encode(text={})", text);
    let mut count = 1;
    let mut encoded: Vec<(char, i32)> = vec![];
	log::debug!("run_length_encode(text={}): count → {}", text, count);

    for (i, c) in text.chars().enumerate() {
        if i + 1 < text.len() && c == text.chars().nth(i + 1).unwrap() {
            count += 1;
        } else {
            encoded.push((c, count));
            count = 1;
        }
    }

	log::debug!("<<< Exiting run_length_encode(text={})", text);
    encoded
}

pub fn run_length_decode(encoded: &[(char, i32)]) -> String {
	log::debug!(">>> Entering run_length_decode(encoded={:?})", encoded);
    let res = encoded
        .iter()
        .map(|x| (x.0).to_string().repeat(x.1 as usize))
	log::debug!("<<< Exiting run_length_decode(encoded={:?})", encoded);
        .collect::<String>();

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_length_decode() {
		log::debug!(">>> Entering test_run_length_decode()");
        let res = run_length_decode(&[('A', 0)]);
        assert_eq!(res, "");
		log::debug!("test_run_length_decode(): res → {}", res);
        let res = run_length_decode(&[('B', 1)]);
        assert_eq!(res, "B");
        let res = run_length_decode(&[('A', 5), ('z', 3), ('B', 1)]);
		log::debug!("<<< Exiting test_run_length_decode()");
        assert_eq!(res, "AAAAAzzzB");
    }

    #[test]
    fn test_run_length_encode() {
		log::debug!(">>> Entering test_run_length_encode()");
        let res = run_length_encode("");
        assert_eq!(res, []);
		log::debug!("test_run_length_encode(): res → {:?}", res);

        let res = run_length_encode("A");
        assert_eq!(res, [('A', 1)]);

        let res = run_length_encode("AA");
        assert_eq!(res, [('A', 2)]);

        let res = run_length_encode("AAAABBBCCDAA");
        assert_eq!(res, [('A', 4), ('B', 3), ('C', 2), ('D', 1), ('A', 2)]);

        let res = run_length_encode("Rust-Trends");
        assert_eq!(
            res,
            [
                ('R', 1),
                ('u', 1),
                ('s', 1),
                ('t', 1),
                ('-', 1),
                ('T', 1),
                ('r', 1),
                ('e', 1),
                ('n', 1),
                ('d', 1),
                ('s', 1)
            ]
		log::debug!("<<< Exiting test_run_length_encode()");
        );
    }
}
