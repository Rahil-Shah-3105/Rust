/// This function returns the absolute value of a number.\
/// The absolute value of a number is the non-negative value of the number, regardless of its sign.\
///
/// Wikipedia: <https://en.wikipedia.org/wiki/Absolute_value>
pub fn abs<T>(num: T) -> T
where
    T: std::ops::Neg<Output = T> + PartialOrd + Copy + num_traits::Zero,
{
    if num < T::zero() {
		log::debug!(">>> Entering abs(num=(unprintable))");
        return -num;
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negative_number_i32() {
		log::debug!(">>> Entering test_negative_number_i32()");
		log::debug!("<<< Exiting test_negative_number_i32()");
        assert_eq!(69, abs(-69));
    }

    #[test]
    fn test_negative_number_f64() {
		log::debug!(">>> Entering test_negative_number_f64()");
		log::debug!("<<< Exiting test_negative_number_f64()");
        assert_eq!(69.69, abs(-69.69));
    }

    #[test]
    fn zero() {
		log::debug!(">>> Entering zero()");
		log::debug!("<<< Exiting zero()");
        assert_eq!(0.0, abs(0.0));
    }

    #[test]
    fn positive_number() {
		log::debug!(">>> Entering positive_number()");
		log::debug!("<<< Exiting positive_number()");
        assert_eq!(69.69, abs(69.69));
    }
}
