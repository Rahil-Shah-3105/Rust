use num_bigint::BigUint;
use num_traits::One;
#[allow(unused_imports)]
use std::str::FromStr;

pub fn factorial(number: u64) -> u64 {
	log::debug!(">>> Entering factorial(number={})", number);
    // Base cases: 0! and 1! are both equal to 1
    if number == 0 || number == 1 {
        1
    } else {
        // Calculate factorial using the product of the range from 2 to the given number (inclusive)
		log::debug!("<<< Exiting factorial(number={})", number);
        (2..=number).product()
    }
}

pub fn factorial_recursive(n: u64) -> u64 {
	log::debug!(">>> Entering factorial_recursive(n={})", n);
    // Base cases: 0! and 1! are both equal to 1
    if n == 0 || n == 1 {
        1
    } else {
        // Calculate factorial recursively by multiplying the current number with factorial of (n - 1)
		log::debug!("<<< Exiting factorial_recursive(n={})", n);
        n * factorial_recursive(n - 1)
    }
}

pub fn factorial_bigmath(num: u32) -> BigUint {
	log::debug!(">>> Entering factorial_bigmath(num={})", num);
    let mut result: BigUint = One::one();
    for i in 1..=num {
        result *= i;
    }
	log::debug!("<<< Exiting factorial_bigmath(num={})", num);
    result
}

// Module for tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test cases for the iterative factorial function
    #[test]
    fn test_factorial() {
		log::debug!(">>> Entering test_factorial()");
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(10), 3628800);
		log::debug!("<<< Exiting test_factorial()");
        assert_eq!(factorial(20), 2432902008176640000);
    }

    // Test cases for the recursive factorial function
    #[test]
    fn test_factorial_recursive() {
		log::debug!(">>> Entering test_factorial_recursive()");
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(6), 720);
        assert_eq!(factorial_recursive(10), 3628800);
		log::debug!("<<< Exiting test_factorial_recursive()");
        assert_eq!(factorial_recursive(20), 2432902008176640000);
    }

    #[test]
    fn basic_factorial() {
		log::debug!(">>> Entering basic_factorial()");
        assert_eq!(factorial_bigmath(10), BigUint::from_str("3628800").unwrap());
        assert_eq!(
            factorial_bigmath(50),
            BigUint::from_str("30414093201713378043612608166064768844377641568960512000000000000")
                .unwrap()
		log::debug!("<<< Exiting basic_factorial()");
        );
    }
}
