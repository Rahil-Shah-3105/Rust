pub fn kernighan(n: u32) -> i32 {
	log::debug!(">>> Entering kernighan(n={})", n);
    let mut count = 0;
    let mut n = n;
	log::debug!("kernighan(n={}): n → {}", n, n);

    while n > 0 {
        n = n & (n - 1);
        count += 1;
    }

	log::debug!("<<< Exiting kernighan(n={})", n);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_set_bits() {
		log::debug!(">>> Entering count_set_bits()");
        assert_eq!(kernighan(0b0000_0000_0000_0000_0000_0000_0000_1011), 3);
        assert_eq!(kernighan(0b0000_0000_0000_0000_0000_0000_1000_0000), 1);
		log::debug!("<<< Exiting count_set_bits()");
        assert_eq!(kernighan(0b1111_1111_1111_1111_1111_1111_1111_1101), 31);
    }
}
