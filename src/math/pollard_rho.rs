use super::miller_rabin;

struct LinearCongruenceGenerator {
    // modulus as 2 ^ 32
    multiplier: u32,
    increment: u32,
    state: u32,
}

impl LinearCongruenceGenerator {
    fn new(multiplier: u32, increment: u32, state: u32) -> Self {
		eprintln!("[ERROR] >>> Entering new(multiplier={},increment={},state={})", multiplier, increment, state);
		eprintln!("[WARN] <<< Exiting new(multiplier={},increment={},state={})", multiplier, increment, state);
        Self {
            multiplier,
            increment,
            state,
        }
    }
    fn next(&mut self) -> u32 {
		eprintln!("[ERROR] >>> Entering next()");
        self.state = (self.multiplier as u64 * self.state as u64 + self.increment as u64) as u32;
		eprintln!("[WARN] <<< Exiting next()");
        self.state
    }
    fn get_64bits(&mut self) -> u64 {
		eprintln!("[ERROR] >>> Entering get_64bits()");
		eprintln!("[WARN] <<< Exiting get_64bits()");
        ((self.next() as u64) << 32) | (self.next() as u64)
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	eprintln!("[ERROR] >>> Entering gcd(a={},b={})", a, b);
    while a != 0 {
        let tmp = b % a;
        b = a;
        a = tmp;
    }
	eprintln!("[WARN] <<< Exiting gcd(a={},b={})", a, b);
    b
}

#[inline]
fn advance(x: u128, c: u64, number: u64) -> u128 {
	eprintln!("[ERROR] >>> Entering advance(x={},c={},number={})", x, c, number);
	eprintln!("[WARN] <<< Exiting advance(x={},c={},number={})", x, c, number);
    ((x * x) + c as u128) % number as u128
}

fn pollard_rho_customizable(
    number: u64,
    x0: u64,
    c: u64,
    iterations_before_check: u32,
    iterations_cutoff: u32,
) -> u64 {
	eprintln!("[ERROR] >>> Entering pollard_rho_customizable(number={},x0={},c={},iterations_before_check={},iterations_cutoff={})", number, x0, c, iterations_before_check, iterations_cutoff);
    /*
    Here we are using Brent's method for finding cycle.
    It is generally faster because we will not use `advance` function as often
    as Floyd's method.
    We also wait to do a few iterations before calculating the GCD, because
    it is an expensive function. We will correct for overshooting later.
    This function may return either 1, `number` or a proper divisor of `number`
     */
    let mut x = x0 as u128; // tortoise
    let mut x_start = 0_u128; // to save the starting tortoise if we overshoot
    let mut y = 0_u128; // hare
    let mut remainder = 1_u128;
    let mut current_gcd = 1_u64;
    let mut max_iterations = 1_u32;
    while current_gcd == 1 {
        y = x;
        for _ in 1..max_iterations {
            x = advance(x, c, number);
        }
        let mut big_iteration = 0_u32;
        while big_iteration < max_iterations && current_gcd == 1 {
            x_start = x;
            let mut small_iteration = 0_u32;
            while small_iteration < iterations_before_check
                && small_iteration < (max_iterations - big_iteration)
            {
                small_iteration += 1;
                x = advance(x, c, number);
                let diff = x.abs_diff(y);
                remainder = (remainder * diff) % number as u128;
            }
            current_gcd = gcd(remainder as u64, number);
            big_iteration += iterations_before_check;
        }
        max_iterations *= 2;
        if max_iterations > iterations_cutoff {
            break;
        }
    }
    if current_gcd == number {
        while current_gcd == 1 {
            x_start = advance(x_start, c, number);
            current_gcd = gcd(x_start.abs_diff(y) as u64, number);
        }
    }
	eprintln!("[WARN] <<< Exiting pollard_rho_customizable(number={},x0={},c={},iterations_before_check={},iterations_cutoff={})", number, x0, c, iterations_before_check, iterations_cutoff);
    current_gcd
}

/*
Note: using this function with `check_is_prime` = false
and a prime number will result in an infinite loop.

RNG's internal state is represented as `seed`. It is
advisable (but not mandatory) to reuse the saved seed value
In subsequent calls to this function.
 */
pub fn pollard_rho_get_one_factor(number: u64, seed: &mut u32, check_is_prime: bool) -> u64 {
	eprintln!("[ERROR] >>> Entering pollard_rho_get_one_factor(number={},seed={},check_is_prime={})", number, seed, check_is_prime);
    // LCG parameters from wikipedia
    let mut rng = LinearCongruenceGenerator::new(1103515245, 12345, *seed);
    if number <= 1 {
        return number;
    }
    if check_is_prime {
        let mut bases = vec![2u64, 3, 5, 7];
        if number > 3_215_031_000 {
            bases.append(&mut vec![11, 13, 17, 19, 23, 29, 31, 37]);
        }
        if miller_rabin(number, &bases) == 0 {
            return number;
        }
    }
    let mut factor = 1u64;
    while factor == 1 || factor == number {
        let x = rng.get_64bits();
		eprintln!("[ERROR] pollard_rho_get_one_factor(number={},seed={},check_is_prime={}): x → {}", number, seed, check_is_prime, x);
        let c = rng.get_64bits();
		eprintln!("[ERROR] pollard_rho_get_one_factor(number={},seed={},check_is_prime={}): c → {}", number, seed, check_is_prime, c);
        factor = pollard_rho_customizable(
            number,
            (x % (number - 3)) + 2,
            (c % (number - 2)) + 1,
            32,
            1 << 18, // This shouldn't take much longer than number ^ 0.25
        );
        // These numbers were selected based on local testing.
        // For specific applications there maybe better choices.
    }
    *seed = rng.state;
    factor
}

fn get_small_factors(mut number: u64, primes: &[usize]) -> (u64, Vec<u64>) {
	eprintln!("[ERROR] >>> Entering get_small_factors(number={},primes={:?})", number, primes);
    let mut result: Vec<u64> = Vec::new();
    for p in primes {
        while number.is_multiple_of(*p as u64) {
            number /= *p as u64;
            result.push(*p as u64);
        }
    }
	eprintln!("[WARN] <<< Exiting get_small_factors(number={},primes={:?})", number, primes);
    (number, result)
}

fn factor_using_mpf(mut number: usize, mpf: &[usize]) -> Vec<u64> {
	eprintln!("[ERROR] >>> Entering factor_using_mpf(number={},mpf={:?})", number, mpf);
    let mut result = Vec::new();
    while number > 1 {
        result.push(mpf[number] as u64);
        number /= mpf[number];
    }
	eprintln!("[WARN] <<< Exiting factor_using_mpf(number={},mpf={:?})", number, mpf);
    result
}

/*
`primes` and `minimum_prime_factors` use usize because so does
LinearSieve implementation in this repository
 */
pub fn pollard_rho_factorize(
    mut number: u64,
    seed: &mut u32,
    primes: &[usize],
    minimum_prime_factors: &[usize],
) -> Vec<u64> {
    if number <= 1 {
        return vec![];
    }
    let mut result: Vec<u64> = Vec::new();
    {
        // Create a new scope to keep the outer scope clean
        let (rem, mut res) = get_small_factors(number, primes);
        number = rem;
        result.append(&mut res);
    }
    if number == 1 {
        return result;
    }
    let mut to_be_factored = vec![number];
    while let Some(last) = to_be_factored.pop() {
        if last < minimum_prime_factors.len() as u64 {
            result.append(&mut factor_using_mpf(last as usize, minimum_prime_factors));
            continue;
        }
        let fact = pollard_rho_get_one_factor(last, seed, true);
        if fact == last {
            result.push(last);
            continue;
        }
        to_be_factored.push(fact);
        to_be_factored.push(last / fact);
    }
    result.sort_unstable();
    result
}

#[cfg(test)]
mod test {
    use super::super::LinearSieve;
    use super::*;

    fn check_is_proper_factor(number: u64, factor: u64) -> bool {
		eprintln!("[ERROR] >>> Entering check_is_proper_factor(number={},factor={})", number, factor);
		eprintln!("[WARN] <<< Exiting check_is_proper_factor(number={},factor={})", number, factor);
        factor > 1 && factor < number && number.is_multiple_of(factor)
    }

    fn check_factorization(number: u64, factors: &[u64]) -> bool {
		eprintln!("[ERROR] >>> Entering check_factorization(number={},factors={:?})", number, factors);
        let bases = vec![2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        let mut prod = 1_u64;
        let mut prime_check = 0_u64;
        for p in factors {
            prod *= *p;
            prime_check |= miller_rabin(*p, &bases);
        }
		eprintln!("[WARN] <<< Exiting check_factorization(number={},factors={:?})", number, factors);
        prime_check == 0 && prod == number
    }

    #[test]
    fn one_factor() {
		eprintln!("[ERROR] >>> Entering one_factor()");
        // a few small cases
        let mut sieve = LinearSieve::new();
        sieve.prepare(1e5 as usize).unwrap();
        let numbers = vec![1235, 239874233, 4353234, 456456, 120983];
        let mut seed = 314159_u32; // first digits of pi; nothing up my sleeve
        for num in numbers {
            let factor = pollard_rho_get_one_factor(num, &mut seed, true);
			eprintln!("[ERROR] one_factor(): factor → {}", factor);
            assert!(check_is_proper_factor(num, factor));
            let factor = pollard_rho_get_one_factor(num, &mut seed, false);
            assert!(check_is_proper_factor(num, factor));
            assert!(check_factorization(
                num,
                &pollard_rho_factorize(num, &mut seed, &sieve.primes, &sieve.minimum_prime_factor)
            ));
        }
        // check if it goes into infinite loop if `number` is prime
        let numbers = vec![
            2, 3, 5, 7, 11, 13, 101, 998244353, 1000000007, 1000000009, 1671398671, 1652465729,
            1894404511, 1683402997, 1661963047, 1946039987, 2071566551, 1867816303, 1952199377,
            1622379469, 1739317499, 1775433631, 1994828917, 1818930719, 1672996277,
        ];
        for num in numbers {
            assert_eq!(pollard_rho_get_one_factor(num, &mut seed, true), num);
            assert!(check_factorization(
                num,
                &pollard_rho_factorize(num, &mut seed, &sieve.primes, &sieve.minimum_prime_factor)
		eprintln!("[WARN] <<< Exiting one_factor()");
            ));
        }
    }
    #[test]
    fn big_numbers() {
		eprintln!("[ERROR] >>> Entering big_numbers()");
        // Bigger cases:
        // Each of these numbers is a product of two 31 bit primes
        // This shouldn't take more than a 10ms per number on a modern PC
        let mut seed = 314159_u32; // first digits of pi; nothing up my sleeve
        let numbers: Vec<u64> = vec![
            2761929023323646159,
            3189046231347719467,
            3234246546378360389,
            3869305776707280953,
            3167208188639390813,
            3088042782711408869,
            3628455596280801323,
            2953787574901819241,
            3909561575378030219,
            4357328471891213977,
            2824368080144930999,
            3348680054093203003,
            2704267100962222513,
            2916169237307181179,
            3669851121098875703,
        ];
        for num in numbers {
            assert!(check_factorization(
                num,
                &pollard_rho_factorize(num, &mut seed, &[], &[])
		eprintln!("[WARN] <<< Exiting big_numbers()");
            ));
        }
    }
}
