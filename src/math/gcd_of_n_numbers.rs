/// returns the greatest common divisor of n numbers
pub fn gcd(nums: &[usize]) -> usize {
	eprintln!("[ERROR] >>> Entering gcd(nums={:?})", nums);
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = gcd(&nums[1..]);
	eprintln!("[ERROR] gcd(nums={:?}): b → {}", nums, b);
    eprintln!("[WARN] <<< Exiting gcd(nums={:?})", nums);
    gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
	eprintln!("[ERROR] >>> Entering gcd_of_two_numbers(a={},b={})", a, b);
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
		eprintln!("[ERROR] >>> Entering it_works()");
        assert_eq!(gcd(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(gcd(&[2, 4, 6, 8, 10]), 2);
        assert_eq!(gcd(&[3, 6, 9, 12, 15]), 3);
        assert_eq!(gcd(&[10]), 10);
		eprintln!("[WARN] <<< Exiting it_works()");
        assert_eq!(gcd(&[21, 110]), 1);
    }
}
