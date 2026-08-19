pub fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
	log::debug!(">>> Entering find_root()");
    let mut result = guess;
    for _ in 0..iterations {
        result = iteration(f, fd, result);
    }
	log::debug!("<<< Exiting find_root()");
    result
}

pub fn iteration(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64) -> f64 {
	log::debug!(">>> Entering iteration()");
    guess - f(guess) / fd(guess)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn math_fn(x: f64) -> f64 {
		log::debug!(">>> Entering math_fn(x={})", x);
        x.cos() - (x * x * x)
    }
    fn math_fnd(x: f64) -> f64 {
        -x.sin() - 3.0 * (x * x)
    }
    #[test]
    fn basic() {
		log::debug!(">>> Entering basic()");
		log::debug!("<<< Exiting basic()");
        assert_eq!(find_root(math_fn, math_fnd, 0.5, 6), 0.8654740331016144);
    }
}
