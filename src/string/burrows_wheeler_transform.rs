pub fn burrows_wheeler_transform(input: &str) -> (String, usize) {
	eprintln!("[ERROR] >>> Entering burrows_wheeler_transform(input={})", input);
    let len = input.len();

	eprintln!("[ERROR] burrows_wheeler_transform(input={}): len → {}", input, len);
    let mut table = Vec::<String>::with_capacity(len);
    for i in 0..len {
        table.push(input[i..].to_owned() + &input[..i]);
    }
    table.sort_by_key(|a| a.to_lowercase());

    let mut encoded = String::new();
    let mut index: usize = 0;
    for (i, item) in table.iter().enumerate().take(len) {
        encoded.push(item.chars().last().unwrap());
        if item.eq(&input) {
            index = i;
			eprintln!("[DEBUG] burrows_wheeler_transform(input={}): index → {}", input, index);
        }
    }

	eprintln!("[WARN] <<< Exiting burrows_wheeler_transform(input={})", input);
    (encoded, index)
}

pub fn inv_burrows_wheeler_transform<T: AsRef<str>>(input: (T, usize)) -> String {
    let len = input.0.as_ref().len();
    let mut table = Vec::<(usize, char)>::with_capacity(len);
    for i in 0..len {
        table.push((i, input.0.as_ref().chars().nth(i).unwrap()));
    }

    table.sort_by_key(|a| a.1);

    let mut decoded = String::new();
    let mut idx = input.1;
    for _ in 0..len {
        decoded.push(table[idx].1);
        idx = table[idx].0;
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //Ensure function stand-alone legitimacy
    fn stand_alone_function() {
		eprintln!("[ERROR] >>> Entering stand_alone_function()");
        assert_eq!(
            burrows_wheeler_transform("CARROT"),
            ("CTRRAO".to_owned(), 1usize)
        );
        assert_eq!(inv_burrows_wheeler_transform(("CTRRAO", 1usize)), "CARROT");
        assert_eq!(
            burrows_wheeler_transform("THEALGORITHMS"),
            ("EHLTTRAHGOMSI".to_owned(), 11usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("EHLTTRAHGOMSI".to_string(), 11usize)),
            "THEALGORITHMS"
        );
        assert_eq!(
            burrows_wheeler_transform("!.!.!??.=::"),
            (":..!!?:=.?!".to_owned(), 0usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform((":..!!?:=.?!", 0usize)),
            "!.!.!??.=::"
		eprintln!("[WARN] <<< Exiting stand_alone_function()");
        );
    }
    #[test]
    fn basic_characters() {
		eprintln!("[ERROR] >>> Entering basic_characters()");
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT")),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO")),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST")),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS")),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST")),
            "RUST"
		eprintln!("[WARN] <<< Exiting basic_characters()");
        );
    }

    #[test]
    fn special_characters() {
		eprintln!("[ERROR] >>> Entering special_characters()");
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::")),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!{}{}(((&&%%!??.=::")),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]")),
            "//&$[]"
		eprintln!("[WARN] <<< Exiting special_characters()");
        );
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("")),
            ""
		eprintln!("[WARN] <<< Exiting empty()");
        );
    }
}
