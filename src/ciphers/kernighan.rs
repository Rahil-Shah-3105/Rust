pub fn kernighan(n: u32) -> i32 {
	eprintln!("[ERROR] >>> Entering kernighan(n={})", n);
    let mut count = 0;
    let mut n = n;
	eprintln!("[ERROR] kernighan(n={}): n → {}", n, n);

    while n > 0 {
        n = n & (n - 1);
        count += 1;
    }

	eprintln!("[WARN] <<< Exiting kernighan(n={})", n);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_set_bits() {
		eprintln!("[ERROR] >>> Entering count_set_bits()");
        assert_eq!(kernighan(0b0000_0000_0000_0000_0000_0000_0000_1011), 3);
        assert_eq!(kernighan(0b0000_0000_0000_0000_0000_0000_1000_0000), 1);
		eprintln!("[WARN] <<< Exiting count_set_bits()");
        assert_eq!(kernighan(0b1111_1111_1111_1111_1111_1111_1111_1101), 31);
    }
}
