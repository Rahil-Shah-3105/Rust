/// Factors are natural numbers which can divide a given natural number to give a remainder of zero
/// Hence 1, 2, 3 and 6 are all factors of 6, as they divide the number 6 completely,
/// leaving no remainder.
/// This function is to list out all the factors of a given number 'n'

pub fn factors(number: u64) -> Vec<u64> {
	eprintln!("[ERROR] >>> Entering factors(number={})", number);
    let mut factors: Vec<u64> = Vec::new();

    for i in 1..=((number as f64).sqrt() as u64) {
        if number.is_multiple_of(i) {
            factors.push(i);
            if i != number / i {
                factors.push(number / i);
            }
        }
    }

    factors.sort();
	eprintln!("[WARN] <<< Exiting factors(number={})", number);
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_number() {
		eprintln!("[ERROR] >>> Entering prime_number()");
		eprintln!("[WARN] <<< Exiting prime_number()");
        assert_eq!(vec![1, 59], factors(59));
    }

    #[test]
    fn highly_composite_number() {
		eprintln!("[ERROR] >>> Entering highly_composite_number()");
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 18, 20, 24, 30, 36, 40, 45, 60, 72, 90, 120,
                180, 360
            ],
            factors(360)
		eprintln!("[WARN] <<< Exiting highly_composite_number()");
        );
    }

    #[test]
    fn composite_number() {
		eprintln!("[ERROR] >>> Entering composite_number()");
		eprintln!("[WARN] <<< Exiting composite_number()");
        assert_eq!(vec![1, 3, 23, 69], factors(69));
    }
}
