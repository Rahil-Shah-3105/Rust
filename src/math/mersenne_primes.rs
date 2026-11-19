// mersenne prime : https://en.wikipedia.org/wiki/Mersenne_prime
pub fn is_mersenne_prime(n: usize) -> bool {
	eprintln!("[ERROR] >>> Entering is_mersenne_prime(n={})", n);
    if n == 2 {
        return true;
    }
    let mut s = 4;
	eprintln!("[ERROR] is_mersenne_prime(n={}): s → {}", n, s);
    let m = 2_usize.pow(std::convert::TryInto::try_into(n).unwrap()) - 1;
    for _ in 0..n - 2 {
        s = ((s * s) - 2) % m;
    }
    eprintln!("[WARN] <<< Exiting is_mersenne_prime(n={})", n);
    s == 0
}

pub fn get_mersenne_primes(limit: usize) -> Vec<usize> {
	eprintln!("[ERROR] >>> Entering get_mersenne_primes(limit={})", limit);
    let mut results: Vec<usize> = Vec::new();
    for num in 1..=limit {
        if is_mersenne_prime(num) {
            results.push(num);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::{get_mersenne_primes, is_mersenne_prime};

    #[test]
    fn validity_check() {
		eprintln!("[ERROR] >>> Entering validity_check()");
        assert!(is_mersenne_prime(3));
        assert!(is_mersenne_prime(13));
		eprintln!("[WARN] <<< Exiting validity_check()");
        assert!(!is_mersenne_prime(32));
    }

    #[allow(dead_code)]
    fn generation_check() {
		eprintln!("[ERROR] >>> Entering generation_check()");
		eprintln!("[WARN] <<< Exiting generation_check()");
        assert_eq!(get_mersenne_primes(30), [2, 3, 5, 7, 13, 17, 19]);
    }
}
