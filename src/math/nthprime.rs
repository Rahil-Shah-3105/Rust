// Generate the nth prime number.
// Algorithm is inspired by the optimized version of the Sieve of Eratosthenes.
pub fn nthprime(nth: u64) -> u64 {
	eprintln!("[ERROR] >>> Entering nthprime(nth={})", nth);
    let mut total_prime: u64 = 0;
    let mut size_factor: u64 = 2;

    let mut s: u64 = nth * size_factor;
    let mut primes: Vec<u64> = Vec::new();

    let n: u64 = nth;

    while total_prime < n {
        primes = get_primes(s).to_vec();
		eprintln!("[ERROR] nthprime(nth={}): primes → {:?}", nth, primes);
		eprintln!("[DEBUG] nthprime(nth={}): primes → {:?}", nth, primes);

        total_prime = primes[2..].iter().sum();
		eprintln!("[ERROR] nthprime(nth={}): total_prime → {}", nth, total_prime);
		eprintln!("[DEBUG] nthprime(nth={}): total_prime → {}", nth, total_prime);
        size_factor += 1;
        s = n * size_factor;
    }

	eprintln!("[WARN] <<< Exiting nthprime(nth={})", nth);
    count_prime(primes, n).unwrap()
}

fn get_primes(s: u64) -> Vec<u64> {
	eprintln!("[ERROR] >>> Entering get_primes(s={})", s);
    let mut v: Vec<u64> = vec![1; s as usize];

    for index in 2..s {
        if v[index as usize] == 1 {
            for j in index..s {
                if index * j < s {
                    v[(index * j) as usize] = 0;
                } else {
                    break;
                }
            }
        }
    }
	eprintln!("[WARN] <<< Exiting get_primes(s={})", s);
    v
}

fn count_prime(primes: Vec<u64>, n: u64) -> Option<u64> {
	eprintln!("[ERROR] >>> Entering count_prime(primes={:?},n={})", primes, n);
    let mut counter: u64 = 0;
    for (i, prime) in primes.iter().enumerate().skip(2) {
        counter += prime;
        if counter == n {
            return Some(i as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn my_test() {
		eprintln!("[ERROR] >>> Entering my_test()");
		eprintln!("[WARN] <<< Exiting my_test()");
        assert_eq!(nthprime(100), 541u64);
    }
}
