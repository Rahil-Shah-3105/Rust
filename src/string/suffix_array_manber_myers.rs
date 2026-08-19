pub fn generate_suffix_array_manber_myers(input: &str) -> Vec<usize> {
	log::debug!(">>> Entering generate_suffix_array_manber_myers(input={})", input);
    if input.is_empty() {
        return Vec::new();
    }
    let n = input.len();
	log::debug!("generate_suffix_array_manber_myers(input={}): n → {}", input, n);
    let mut suffixes: Vec<(usize, &str)> = Vec::with_capacity(n);

    for (i, _suffix) in input.char_indices() {
        suffixes.push((i, &input[i..]));
    }

    suffixes.sort_by_key(|&(_, s)| s);

    let mut suffix_array: Vec<usize> = vec![0; n];
    let mut rank = vec![0; n];

    let mut cur_rank = 0;
    let mut prev_suffix = &suffixes[0].1;
	log::debug!("generate_suffix_array_manber_myers(input={}): prev_suffix → (unprintable)", input);

    for (i, suffix) in suffixes.iter().enumerate() {
        if &suffix.1 != prev_suffix {
            cur_rank += 1;
            prev_suffix = &suffix.1;
        }
        rank[suffix.0] = cur_rank;
        suffix_array[i] = suffix.0;
    }

    let mut k = 1;
    let mut new_rank: Vec<usize> = vec![0; n];

    while k < n {
        suffix_array.sort_by_key(|&x| (rank[x], rank[(x + k) % n]));

        let mut cur_rank = 0;
        let mut prev = suffix_array[0];
		log::debug!("generate_suffix_array_manber_myers(input={}): prev → (unprintable)", input);
        new_rank[prev] = cur_rank;

        for &suffix in suffix_array.iter().skip(1) {
            let next = suffix;
            if (rank[prev], rank[(prev + k) % n]) != (rank[next], rank[(next + k) % n]) {
                cur_rank += 1;
            }
            new_rank[next] = cur_rank;
            prev = next;
        }

        std::mem::swap(&mut rank, &mut new_rank);

        k <<= 1;
    }

    log::debug!("<<< Exiting generate_suffix_array_manber_myers(input={})", input);
    suffix_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
		log::debug!(">>> Entering test_suffix_array()");
        let input = "banana";
        let expected_result = vec![5, 3, 1, 0, 4, 2];
		log::debug!("test_suffix_array(): expected_result → (unprintable)");
		log::debug!("test_suffix_array(): input → {}", input);
    log::debug!("<<< Exiting test_suffix_array()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_empty_string() {
		log::debug!(">>> Entering test_empty_string()");
        let input = "";
        let expected_result: Vec<usize> = Vec::new();
		log::debug!("test_empty_string(): input → {}", input);
		log::debug!("<<< Exiting test_empty_string()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_single_character() {
		log::debug!(">>> Entering test_single_character()");
        let input = "a";
        let expected_result = vec![0];
		log::debug!("test_single_character(): expected_result → (unprintable)");
		log::debug!("test_single_character(): input → {}", input);
    log::debug!("<<< Exiting test_single_character()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }
    #[test]
    fn test_repeating_characters() {
		log::debug!(">>> Entering test_repeating_characters()");
        let input = "zzzzzz";
        let expected_result = vec![5, 4, 3, 2, 1, 0];
		log::debug!("test_repeating_characters(): expected_result → (unprintable)");
		log::debug!("test_repeating_characters(): input → {}", input);
    log::debug!("<<< Exiting test_repeating_characters()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_long_string() {
		log::debug!(">>> Entering test_long_string()");
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected_result: Vec<usize> = (0..26).collect();
		log::debug!("test_long_string(): input → {}", input);
		log::debug!("<<< Exiting test_long_string()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_mix_of_characters() {
		log::debug!(">>> Entering test_mix_of_characters()");
        let input = "abracadabra!";
        let expected_result = vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
		log::debug!("test_mix_of_characters(): expected_result → (unprintable)");
		log::debug!("test_mix_of_characters(): input → {}", input);
    log::debug!("<<< Exiting test_mix_of_characters()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_whitespace_characters() {
		log::debug!(">>> Entering test_whitespace_characters()");
        let input = " hello world ";
        let expected_result = vec![12, 0, 6, 11, 2, 1, 10, 3, 4, 5, 8, 9, 7];
		log::debug!("test_whitespace_characters(): expected_result → (unprintable)");
		log::debug!("test_whitespace_characters(): input → {}", input);
    log::debug!("<<< Exiting test_whitespace_characters()");
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }
}
