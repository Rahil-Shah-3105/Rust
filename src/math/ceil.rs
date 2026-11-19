// In mathematics and computer science, the ceiling function maps x to the least integer greater than or equal to x
// Source: https://en.wikipedia.org/wiki/Floor_and_ceiling_functions

pub fn ceil(x: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering ceil(x={})", x);
    let x_rounded_towards_zero = x as i32 as f64;
    if x < 0. || x_rounded_towards_zero == x {
        x_rounded_towards_zero
    } else {
		eprintln!("[WARN] <<< Exiting ceil(x={})", x);
        x_rounded_towards_zero + 1_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimal() {
		eprintln!("[ERROR] >>> Entering positive_decimal()");
        let num = 1.10;
		eprintln!("[WARN] <<< Exiting positive_decimal()");
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_decimal_with_small_number() {
		eprintln!("[ERROR] >>> Entering positive_decimal_with_small_number()");
        let num = 3.01;
		eprintln!("[WARN] <<< Exiting positive_decimal_with_small_number()");
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_integer() {
		eprintln!("[ERROR] >>> Entering positive_integer()");
        let num = 1.00;
		eprintln!("[WARN] <<< Exiting positive_integer()");
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal() {
		eprintln!("[ERROR] >>> Entering negative_decimal()");
        let num = -1.10;
		eprintln!("[WARN] <<< Exiting negative_decimal()");
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal_with_small_number() {
		eprintln!("[ERROR] >>> Entering negative_decimal_with_small_number()");
        let num = -1.01;
		eprintln!("[WARN] <<< Exiting negative_decimal_with_small_number()");
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_integer() {
		eprintln!("[ERROR] >>> Entering negative_integer()");
        let num = -1.00;
		eprintln!("[WARN] <<< Exiting negative_integer()");
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn zero() {
		eprintln!("[ERROR] >>> Entering zero()");
        let num = 0.00;
		eprintln!("[WARN] <<< Exiting zero()");
        assert_eq!(ceil(num), num.ceil());
    }
}
