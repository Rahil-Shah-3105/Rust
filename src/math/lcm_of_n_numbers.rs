// returns the least common multiple of n numbers

pub fn lcm(nums: &[usize]) -> usize {
	eprintln!("[ERROR] >>> Entering lcm(nums={:?})", nums);
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
	eprintln!("[ERROR] lcm(nums={:?}): b → {}", nums, b);
    eprintln!("[WARN] <<< Exiting lcm(nums={:?})", nums);
    a * b / gcd_of_two_numbers(a, b)
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
        assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2, 4, 6, 8, 10]), 120);
        assert_eq!(lcm(&[3, 6, 9, 12, 15]), 180);
        assert_eq!(lcm(&[10]), 10);
		eprintln!("[WARN] <<< Exiting it_works()");
        assert_eq!(lcm(&[21, 110]), 2310);
    }
}
