pub fn decimal_to_binary(base_num: u64) -> String {
	log::debug!(">>> Entering decimal_to_binary(base_num={})", base_num);
    let mut num = base_num;
    let mut binary_num = String::new();
    loop {
        let bit = (num % 2).to_string();
		log::debug!("decimal_to_binary(base_num={}): bit → {}", base_num, bit);
        binary_num.push_str(&bit);
        num /= 2;
        if num == 0 {
            break;
        }
    }

    let bits = binary_num.chars();
	log::debug!("decimal_to_binary(base_num={}): bits → (unprintable)", base_num);
    log::debug!("<<< Exiting decimal_to_binary(base_num={})", base_num);
    bits.rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_decimal_to_binary() {
		log::debug!(">>> Entering converting_decimal_to_binary()");
        assert_eq!(decimal_to_binary(542), "1000011110");
		log::debug!("<<< Exiting converting_decimal_to_binary()");
        assert_eq!(decimal_to_binary(92), "1011100");
    }
}
