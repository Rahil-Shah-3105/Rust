// https://en.wikipedia.org/wiki/Square_pyramidal_number
// 1² + 2² + ... = ... (total)

pub fn square_pyramidal_number(n: u64) -> u64 {
	eprintln!("[ERROR] >>> Entering square_pyramidal_number(n={})", n);
	eprintln!("[WARN] <<< Exiting square_pyramidal_number(n={})", n);
    n * (n + 1) * (2 * n + 1) / 6
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test0() {
		eprintln!("[ERROR] >>> Entering test0()");
        assert_eq!(0, square_pyramidal_number(0));
        assert_eq!(1, square_pyramidal_number(1));
        assert_eq!(5, square_pyramidal_number(2));
		eprintln!("[WARN] <<< Exiting test0()");
        assert_eq!(14, square_pyramidal_number(3));
    }
}
