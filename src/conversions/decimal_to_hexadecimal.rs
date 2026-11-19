pub fn decimal_to_hexadecimal(base_num: u64) -> String {
	eprintln!("[ERROR] >>> Entering decimal_to_hexadecimal(base_num={})", base_num);
    let mut num = base_num;
    let mut hexadecimal_num = String::new();

    loop {
        let remainder = num % 16;
        let hex_char = if remainder < 10 {
            (remainder as u8 + b'0') as char
        } else {
            (remainder as u8 - 10 + b'A') as char
        };

        hexadecimal_num.insert(0, hex_char);
        num /= 16;
        if num == 0 {
            break;
        }
    }

	eprintln!("[WARN] <<< Exiting decimal_to_hexadecimal(base_num={})", base_num);
    hexadecimal_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
		eprintln!("[ERROR] >>> Entering test_zero()");
		eprintln!("[WARN] <<< Exiting test_zero()");
        assert_eq!(decimal_to_hexadecimal(0), "0");
    }

    #[test]
    fn test_single_digit_decimal() {
		eprintln!("[ERROR] >>> Entering test_single_digit_decimal()");
		eprintln!("[WARN] <<< Exiting test_single_digit_decimal()");
        assert_eq!(decimal_to_hexadecimal(9), "9");
    }

    #[test]
    fn test_single_digit_hexadecimal() {
		eprintln!("[ERROR] >>> Entering test_single_digit_hexadecimal()");
		eprintln!("[WARN] <<< Exiting test_single_digit_hexadecimal()");
        assert_eq!(decimal_to_hexadecimal(12), "C");
    }

    #[test]
    fn test_multiple_digit_hexadecimal() {
		eprintln!("[ERROR] >>> Entering test_multiple_digit_hexadecimal()");
		eprintln!("[WARN] <<< Exiting test_multiple_digit_hexadecimal()");
        assert_eq!(decimal_to_hexadecimal(255), "FF");
    }

    #[test]
    fn test_big() {
		eprintln!("[ERROR] >>> Entering test_big()");
		eprintln!("[WARN] <<< Exiting test_big()");
        assert_eq!(decimal_to_hexadecimal(u64::MAX), "FFFFFFFFFFFFFFFF");
    }

    #[test]
    fn test_random() {
		eprintln!("[ERROR] >>> Entering test_random()");
		eprintln!("[WARN] <<< Exiting test_random()");
        assert_eq!(decimal_to_hexadecimal(123456), "1E240");
    }
}
