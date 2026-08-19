//! Binary Shift Operations
//!
//! This module provides implementations of various binary shift operations with
//! binary string output for visualization.
//!
//! # Shift Types
//!
//! - **Logical Left Shift**: Shifts bits left, filling with zeros on the right
//! - **Logical Right Shift**: Shifts bits right, filling with zeros on the left
//! - **Arithmetic Left Shift**: Same as logical left shift (included for completeness)
//! - **Arithmetic Right Shift**: Shifts bits right, preserving the sign bit
//!
//! # Note on Arithmetic vs Logical Left Shifts
//!
//! In most systems, arithmetic left shift and logical left shift are identical operations.
//! Both shift bits to the left and fill with zeros on the right. The distinction between
//! arithmetic and logical shifts only matters for right shifts, where arithmetic shifts
//! preserve the sign bit.
//!
//! # References
//!
//! - [Bitwise Operations - Python Docs](https://docs.python.org/3/library/stdtypes.html#bitwise-operations-on-integer-types)
//! - [Bit Shift - Interview Cake](https://www.interviewcake.com/concept/java/bit-shift)

/// Performs a logical left shift on a number and returns the binary representation.
///
/// Shifts the bits of `number` to the left by `shift_amount` positions,
/// filling the rightmost bits with zeros.
///
/// # Arguments
///
/// * `number` - The non-negative integer to be shifted
/// * `shift_amount` - The number of positions to shift (must be non-negative)
///
/// # Returns
///
/// `Ok(String)` with the binary representation (including "0b" prefix),
/// or `Err(String)` if either input is negative
///
/// # Example
///
/// ```
/// use the_algorithms_rust::bit_manipulation::logical_left_shift;
///
/// assert_eq!(logical_left_shift(0, 1).unwrap(), "0b00");
/// assert_eq!(logical_left_shift(1, 1).unwrap(), "0b10");
/// assert_eq!(logical_left_shift(1, 5).unwrap(), "0b100000");
/// assert_eq!(logical_left_shift(17, 2).unwrap(), "0b1000100");
/// assert_eq!(logical_left_shift(1983, 4).unwrap(), "0b111101111110000");
///
/// // Negative inputs return error
/// assert!(logical_left_shift(1, -1).is_err());
/// ```
pub fn logical_left_shift(number: i32, shift_amount: i32) -> Result<String, String> {
	log::debug!(">>> Entering logical_left_shift(number={},shift_amount={})", number, shift_amount);
    if number < 0 || shift_amount < 0 {
        return Err("both inputs must be positive integers".to_string());
    }

    // Get binary representation and append zeros
    let binary = format!("{number:b}");
	log::debug!("logical_left_shift(number={},shift_amount={}): binary → {}", number, shift_amount, binary);
    let zeros = "0".repeat(shift_amount as usize);
	log::debug!("logical_left_shift(number={},shift_amount={}): zeros → {}", number, shift_amount, zeros);
    log::debug!("<<< Exiting logical_left_shift(number={},shift_amount={})", number, shift_amount);
    Ok(format!("0b{binary}{zeros}"))
}

/// Performs a logical right shift on a number and returns the binary representation.
///
/// Shifts the bits of `number` to the right by `shift_amount` positions,
/// filling the leftmost bits with zeros. This is an unsigned shift operation.
///
/// # Arguments
///
/// * `number` - The non-negative integer to be shifted
/// * `shift_amount` - The number of positions to shift (must be non-negative)
///
/// # Returns
///
/// `Ok(String)` with the binary representation (including "0b" prefix),
/// or `Err(String)` if either input is negative
///
/// # Example
///
/// ```
/// use the_algorithms_rust::bit_manipulation::logical_right_shift;
///
/// assert_eq!(logical_right_shift(0, 1).unwrap(), "0b0");
/// assert_eq!(logical_right_shift(1, 1).unwrap(), "0b0");
/// assert_eq!(logical_right_shift(1, 5).unwrap(), "0b0");
/// assert_eq!(logical_right_shift(17, 2).unwrap(), "0b100");
/// assert_eq!(logical_right_shift(1983, 4).unwrap(), "0b1111011");
///
/// // Negative inputs return error
/// assert!(logical_right_shift(1, -1).is_err());
/// ```
pub fn logical_right_shift(number: i32, shift_amount: i32) -> Result<String, String> {
	log::debug!(">>> Entering logical_right_shift(number={},shift_amount={})", number, shift_amount);
    if number < 0 || shift_amount < 0 {
        return Err("both inputs must be positive integers".to_string());
    }

    let shifted = (number as u32) >> shift_amount;
    Ok(format!("0b{shifted:b}"))
}

/// Performs an arithmetic right shift on a number and returns the binary representation.
///
/// Shifts the bits of `number` to the right by `shift_amount` positions,
/// preserving the sign bit. For positive numbers, fills with 0s; for negative
/// numbers, fills with 1s (sign extension).
///
/// # Arguments
///
/// * `number` - The integer to be shifted (can be negative)
/// * `shift_amount` - The number of positions to shift (must be non-negative)
///
/// # Returns
///
/// `Ok(String)` with the binary representation including sign bit (with "0b" prefix),
/// or `Err(String)` if shift_amount is negative
///
/// # Example
///
/// ```
/// use the_algorithms_rust::bit_manipulation::arithmetic_right_shift;
///
/// assert_eq!(arithmetic_right_shift(0, 1).unwrap(), "0b00");
/// assert_eq!(arithmetic_right_shift(1, 1).unwrap(), "0b00");
/// assert_eq!(arithmetic_right_shift(-1, 1).unwrap(), "0b11");
/// assert_eq!(arithmetic_right_shift(17, 2).unwrap(), "0b000100");
/// assert_eq!(arithmetic_right_shift(-17, 2).unwrap(), "0b111011");
/// assert_eq!(arithmetic_right_shift(-1983, 4).unwrap(), "0b111110000100");
/// ```
pub fn arithmetic_right_shift(number: i32, shift_amount: i32) -> Result<String, String> {
	log::debug!(">>> Entering arithmetic_right_shift(number={},shift_amount={})", number, shift_amount);
    if shift_amount < 0 {
        return Err("shift amount must be a positive integer".to_string());
    }

    let shift_amount_usize = shift_amount as usize;

    let binary_number = if number >= 0 {
        // Python: binary_number = "0" + str(bin(number)).strip("-")[2:]
        let bin_str = format!("{number:b}");
		log::debug!("arithmetic_right_shift(number={},shift_amount={}): bin_str → {}", number, shift_amount, bin_str);
        format!("0{bin_str}")
    } else {
        // Python: binary_number_length = len(bin(number)[3:])
        // bin(-17) = "-0b10001", [3:] = "10001", length = 5
        let abs_bin = format!("{:b}", number.abs());
		log::debug!("arithmetic_right_shift(number={},shift_amount={}): abs_bin → {}", number, shift_amount, abs_bin);
        let binary_number_length = abs_bin.len();

        // Python: binary_number = bin(abs(number) - (1 << binary_number_length))[3:]
        let abs_num = number.abs();
        let subtracted = abs_num - (1 << binary_number_length);

        // bin() of negative number is "-0b..." so [3:] skips "-0b"
        let bin_result = if subtracted < 0 {
            // For negative result, we need its absolute value binary representation
            // In Python, bin(-15) = "-0b1111", and [3:] = "1111"
            format!("{:b}", subtracted.abs())
        } else {
            format!("{subtracted:b}")
        };

        // Python: binary_number = "1" + "0" * (binary_number_length - len(binary_number)) + binary_number
        let padding = if binary_number_length > bin_result.len() {
            "0".repeat(binary_number_length - bin_result.len())
        } else {
            String::new()
        };

        format!("1{padding}{bin_result}")
    };

    // Python: if shift_amount >= len(binary_number):
    //             return "0b" + binary_number[0] * len(binary_number)
    if shift_amount_usize >= binary_number.len() {
        let sign_char = binary_number.chars().next().unwrap();
        return Ok(format!(
            "0b{}",
            sign_char.to_string().repeat(binary_number.len())
        ));
    }

    // Python: return ("0b" + binary_number[0] * shift_amount +
    //                 binary_number[: len(binary_number) - shift_amount])
    let sign_char = binary_number.chars().next().unwrap();
    let end_idx = binary_number.len() - shift_amount_usize;
    let slice = &binary_number[..end_idx];

    Ok(format!(
        "0b{}{}",
        sign_char.to_string().repeat(shift_amount_usize),
        slice
    ))
}

/// Performs an arithmetic left shift on a number and returns the binary representation.
///
/// **Note**: Arithmetic left shift is identical to logical left shift - both shift bits
/// to the left and fill with zeros on the right. This function is provided for
/// completeness and educational purposes. The distinction between arithmetic and logical
/// shifts only matters for right shifts (sign preservation).
///
/// # Arguments
///
/// * `number` - The integer to be shifted (can be negative)
/// * `shift_amount` - The number of positions to shift (must be non-negative)
///
/// # Returns
///
/// `Ok(String)` with the binary representation (with "0b" prefix),
/// or `Err(String)` if shift_amount is negative
///
/// # Example
///
/// ```
/// use the_algorithms_rust::bit_manipulation::arithmetic_left_shift;
///
/// assert_eq!(arithmetic_left_shift(1, 5).unwrap(), "0b100000");
/// assert_eq!(arithmetic_left_shift(17, 2).unwrap(), "0b1000100");
/// assert_eq!(arithmetic_left_shift(-1, 2).unwrap(), "0b11111111111111111111111111111100");
/// ```
pub fn arithmetic_left_shift(number: i32, shift_amount: i32) -> Result<String, String> {
	log::debug!(">>> Entering arithmetic_left_shift(number={},shift_amount={})", number, shift_amount);
    if shift_amount < 0 {
        return Err("shift amount must be a positive integer".to_string());
    }

    // Arithmetic left shift is the same as logical left shift
    // Both shift left and fill with zeros
    let shifted = (number << shift_amount) as u32;
    let binary = format!("{shifted:b}");
	log::debug!("arithmetic_left_shift(number={},shift_amount={}): binary → {}", number, shift_amount, binary);
    log::debug!("<<< Exiting arithmetic_left_shift(number={},shift_amount={})", number, shift_amount);
    Ok(format!("0b{binary}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Logical Left Shift Tests
    #[test]
    fn test_logical_left_shift_zero() {
		log::debug!(">>> Entering test_logical_left_shift_zero()");
		log::debug!("<<< Exiting test_logical_left_shift_zero()");
        assert_eq!(logical_left_shift(0, 1).unwrap(), "0b00");
    }

    #[test]
    fn test_logical_left_shift_one() {
		log::debug!(">>> Entering test_logical_left_shift_one()");
		log::debug!("<<< Exiting test_logical_left_shift_one()");
        assert_eq!(logical_left_shift(1, 1).unwrap(), "0b10");
    }

    #[test]
    fn test_logical_left_shift_large_shift() {
		log::debug!(">>> Entering test_logical_left_shift_large_shift()");
		log::debug!("<<< Exiting test_logical_left_shift_large_shift()");
        assert_eq!(logical_left_shift(1, 5).unwrap(), "0b100000");
    }

    #[test]
    fn test_logical_left_shift_seventeen() {
		log::debug!(">>> Entering test_logical_left_shift_seventeen()");
		log::debug!("<<< Exiting test_logical_left_shift_seventeen()");
        assert_eq!(logical_left_shift(17, 2).unwrap(), "0b1000100");
    }

    #[test]
    fn test_logical_left_shift_large_number() {
		log::debug!(">>> Entering test_logical_left_shift_large_number()");
		log::debug!("<<< Exiting test_logical_left_shift_large_number()");
        assert_eq!(logical_left_shift(1983, 4).unwrap(), "0b111101111110000");
    }

    #[test]
    fn test_logical_left_shift_negative_number() {
		log::debug!(">>> Entering test_logical_left_shift_negative_number()");
		log::debug!("<<< Exiting test_logical_left_shift_negative_number()");
        assert!(logical_left_shift(-1, 1).is_err());
    }

    #[test]
    fn test_logical_left_shift_negative_shift() {
		log::debug!(">>> Entering test_logical_left_shift_negative_shift()");
		log::debug!("<<< Exiting test_logical_left_shift_negative_shift()");
        assert!(logical_left_shift(1, -1).is_err());
    }

    #[test]
    fn test_logical_left_shift_both_negative() {
		log::debug!(">>> Entering test_logical_left_shift_both_negative()");
		log::debug!("<<< Exiting test_logical_left_shift_both_negative()");
        assert!(logical_left_shift(-1, -1).is_err());
    }

    // Logical Right Shift Tests
    #[test]
    fn test_logical_right_shift_zero() {
		log::debug!(">>> Entering test_logical_right_shift_zero()");
		log::debug!("<<< Exiting test_logical_right_shift_zero()");
        assert_eq!(logical_right_shift(0, 1).unwrap(), "0b0");
    }

    #[test]
    fn test_logical_right_shift_one() {
		log::debug!(">>> Entering test_logical_right_shift_one()");
		log::debug!("<<< Exiting test_logical_right_shift_one()");
        assert_eq!(logical_right_shift(1, 1).unwrap(), "0b0");
    }

    #[test]
    fn test_logical_right_shift_shift_all_bits() {
		log::debug!(">>> Entering test_logical_right_shift_shift_all_bits()");
		log::debug!("<<< Exiting test_logical_right_shift_shift_all_bits()");
        assert_eq!(logical_right_shift(1, 5).unwrap(), "0b0");
    }

    #[test]
    fn test_logical_right_shift_seventeen() {
		log::debug!(">>> Entering test_logical_right_shift_seventeen()");
		log::debug!("<<< Exiting test_logical_right_shift_seventeen()");
        assert_eq!(logical_right_shift(17, 2).unwrap(), "0b100");
    }

    #[test]
    fn test_logical_right_shift_large_number() {
		log::debug!(">>> Entering test_logical_right_shift_large_number()");
		log::debug!("<<< Exiting test_logical_right_shift_large_number()");
        assert_eq!(logical_right_shift(1983, 4).unwrap(), "0b1111011");
    }

    #[test]
    fn test_logical_right_shift_negative_number() {
		log::debug!(">>> Entering test_logical_right_shift_negative_number()");
		log::debug!("<<< Exiting test_logical_right_shift_negative_number()");
        assert!(logical_right_shift(-1, 1).is_err());
    }

    #[test]
    fn test_logical_right_shift_negative_shift() {
		log::debug!(">>> Entering test_logical_right_shift_negative_shift()");
		log::debug!("<<< Exiting test_logical_right_shift_negative_shift()");
        assert!(logical_right_shift(1, -1).is_err());
    }

    #[test]
    fn test_logical_right_shift_both_negative() {
		log::debug!(">>> Entering test_logical_right_shift_both_negative()");
		log::debug!("<<< Exiting test_logical_right_shift_both_negative()");
        assert!(logical_right_shift(-1, -1).is_err());
    }

    // Arithmetic Right Shift Tests
    #[test]
    fn test_arithmetic_right_shift_zero() {
		log::debug!(">>> Entering test_arithmetic_right_shift_zero()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_zero()");
        assert_eq!(arithmetic_right_shift(0, 1).unwrap(), "0b00");
    }

    #[test]
    fn test_arithmetic_right_shift_one() {
		log::debug!(">>> Entering test_arithmetic_right_shift_one()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_one()");
        assert_eq!(arithmetic_right_shift(1, 1).unwrap(), "0b00");
    }

    #[test]
    fn test_arithmetic_right_shift_negative_one() {
		log::debug!(">>> Entering test_arithmetic_right_shift_negative_one()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_negative_one()");
        assert_eq!(arithmetic_right_shift(-1, 1).unwrap(), "0b11");
    }

    #[test]
    fn test_arithmetic_right_shift_seventeen_positive() {
		log::debug!(">>> Entering test_arithmetic_right_shift_seventeen_positive()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_seventeen_positive()");
        assert_eq!(arithmetic_right_shift(17, 2).unwrap(), "0b000100");
    }

    #[test]
    fn test_arithmetic_right_shift_seventeen_negative() {
		log::debug!(">>> Entering test_arithmetic_right_shift_seventeen_negative()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_seventeen_negative()");
        assert_eq!(arithmetic_right_shift(-17, 2).unwrap(), "0b111011");
    }

    #[test]
    fn test_arithmetic_right_shift_large_negative() {
		log::debug!(">>> Entering test_arithmetic_right_shift_large_negative()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_large_negative()");
        assert_eq!(arithmetic_right_shift(-1983, 4).unwrap(), "0b111110000100");
    }

    #[test]
    fn test_arithmetic_right_shift_negative_shift() {
		log::debug!(">>> Entering test_arithmetic_right_shift_negative_shift()");
		log::debug!("<<< Exiting test_arithmetic_right_shift_negative_shift()");
        assert!(arithmetic_right_shift(1, -1).is_err());
    }

    #[test]
    fn test_arithmetic_right_shift_preserves_sign_positive() {
		log::debug!(">>> Entering test_arithmetic_right_shift_preserves_sign_positive()");
        // Positive number should have leading 0
        // 16 = 0b10000, with sign bit = 0b010000, shift right by 2 = 0b000100
        let result = arithmetic_right_shift(16, 2).unwrap();
		log::debug!("test_arithmetic_right_shift_preserves_sign_positive(): result → {}", result);
        assert!(result.starts_with("0b0"));
		log::debug!("<<< Exiting test_arithmetic_right_shift_preserves_sign_positive()");
        assert_eq!(result, "0b000100");
    }

    #[test]
    fn test_arithmetic_right_shift_preserves_sign_negative() {
		log::debug!(">>> Entering test_arithmetic_right_shift_preserves_sign_negative()");
        // Negative number should have leading 1
        let result = arithmetic_right_shift(-16, 2).unwrap();
		log::debug!("test_arithmetic_right_shift_preserves_sign_negative(): result → {}", result);
		log::debug!("<<< Exiting test_arithmetic_right_shift_preserves_sign_negative()");
        assert!(result.starts_with("0b1"));
    }

    #[test]
    fn test_arithmetic_right_shift_large_shift_positive() {
		log::debug!(">>> Entering test_arithmetic_right_shift_large_shift_positive()");
        // Shifting positive number by large amount
        // 1 = 0b1, with sign bit = 0b01 (2 bits)
        // Shift by 10 (>= 2), so return sign bit repeated 2 times = 0b00
		log::debug!("<<< Exiting test_arithmetic_right_shift_large_shift_positive()");
        assert_eq!(arithmetic_right_shift(1, 10).unwrap(), "0b00");
    }

    #[test]
    fn test_arithmetic_right_shift_large_shift_negative() {
		log::debug!(">>> Entering test_arithmetic_right_shift_large_shift_negative()");
        // Shifting negative number by large amount should preserve sign
        // -1 has all 1s, minimal representation with sign bit
        let result = arithmetic_right_shift(-1, 10).unwrap();
		log::debug!("test_arithmetic_right_shift_large_shift_negative(): result → {}", result);
        assert!(result.starts_with("0b1"));
        // All bits should be 1s (sign extended)
		log::debug!("<<< Exiting test_arithmetic_right_shift_large_shift_negative()");
        assert!(result.chars().skip(2).all(|c| c == '1'));
    }

    // Arithmetic Left Shift Tests
    #[test]
    fn test_arithmetic_left_shift_basic() {
		log::debug!(">>> Entering test_arithmetic_left_shift_basic()");
        assert_eq!(arithmetic_left_shift(1, 5).unwrap(), "0b100000");
		log::debug!("<<< Exiting test_arithmetic_left_shift_basic()");
        assert_eq!(arithmetic_left_shift(17, 2).unwrap(), "0b1000100");
    }

    #[test]
    fn test_arithmetic_left_shift_negative() {
		log::debug!(">>> Entering test_arithmetic_left_shift_negative()");
        // Negative numbers in arithmetic left shift
        // -1 << 2 in two's complement
        let result = arithmetic_left_shift(-1, 2).unwrap();
		log::debug!("test_arithmetic_left_shift_negative(): result → {}", result);
        assert!(result.starts_with("0b"));
        // Should contain all 1s followed by 00
		log::debug!("<<< Exiting test_arithmetic_left_shift_negative()");
        assert!(result.ends_with("00"));
    }

    #[test]
    fn test_arithmetic_left_shift_zero() {
		log::debug!(">>> Entering test_arithmetic_left_shift_zero()");
		log::debug!("<<< Exiting test_arithmetic_left_shift_zero()");
        assert_eq!(arithmetic_left_shift(0, 3).unwrap(), "0b0");
    }

    #[test]
    fn test_arithmetic_left_shift_negative_shift() {
		log::debug!(">>> Entering test_arithmetic_left_shift_negative_shift()");
		log::debug!("<<< Exiting test_arithmetic_left_shift_negative_shift()");
        assert!(arithmetic_left_shift(1, -1).is_err());
    }

    #[test]
    fn test_arithmetic_left_shift_same_as_logical() {
		log::debug!(">>> Entering test_arithmetic_left_shift_same_as_logical()");
        // For positive numbers, arithmetic and logical left shifts are identical
        let num = 17;
        let shift = 3;
        let arithmetic = arithmetic_left_shift(num, shift).unwrap();
		log::debug!("test_arithmetic_left_shift_same_as_logical(): arithmetic → {}", arithmetic);
        let logical = logical_left_shift(num, shift).unwrap();
		log::debug!("test_arithmetic_left_shift_same_as_logical(): logical → {}", logical);

        // Parse the binary strings and compare the values
        let arith_val = u32::from_str_radix(&arithmetic[2..], 2).unwrap();
        let logic_val = u32::from_str_radix(&logical[2..], 2).unwrap();
		log::debug!("<<< Exiting test_arithmetic_left_shift_same_as_logical()");
        assert_eq!(arith_val, logic_val);
    }

    #[test]
    fn test_all_shifts_on_same_value() {
		log::debug!(">>> Entering test_all_shifts_on_same_value()");
        let number = 8;
        let shift = 2;

        // 8 (0b1000) << 2 = 32 (0b100000)
        assert_eq!(logical_left_shift(number, shift).unwrap(), "0b100000");
        assert_eq!(arithmetic_left_shift(number, shift).unwrap(), "0b100000");

        // 8 (0b1000) >> 2 = 2 (0b10)
        assert_eq!(logical_right_shift(number, shift).unwrap(), "0b10");

        // 8 (0b1000) >> 2 = 2 (0b010)
		log::debug!("<<< Exiting test_all_shifts_on_same_value()");
        assert_eq!(arithmetic_right_shift(number, shift).unwrap(), "0b00010");
    }
}
