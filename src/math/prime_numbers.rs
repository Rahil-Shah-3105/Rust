pub fn prime_numbers(max: usize) -> Vec<usize> {
	eprintln!("[ERROR] >>> Entering prime_numbers(max={})", max);
    let mut result: Vec<usize> = Vec::new();

    if max >= 2 {
        result.push(2)
    }
    for i in (3..=max).step_by(2) {
        let stop: usize = (i as f64).sqrt() as usize + 1;
        let mut status = true;

        for j in (3..stop).step_by(2) {
            if i % j == 0 {
                status = false;
				eprintln!("[ERROR] prime_numbers(max={}): status → {}", max, status);
                break;
            }
        }
        if status {
            result.push(i)
        }
    }

	eprintln!("[WARN] <<< Exiting prime_numbers(max={})", max);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
		eprintln!("[ERROR] >>> Entering basic()");
        assert_eq!(prime_numbers(0), vec![]);
        assert_eq!(prime_numbers(11), vec![2, 3, 5, 7, 11]);
        assert_eq!(prime_numbers(25), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(
            prime_numbers(33),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
		eprintln!("[WARN] <<< Exiting basic()");
        );
    }
}
