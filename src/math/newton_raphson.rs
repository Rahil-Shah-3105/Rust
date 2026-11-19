pub fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
	eprintln!("[ERROR] >>> Entering find_root()");
    let mut result = guess;
    for _ in 0..iterations {
        result = iteration(f, fd, result);
    }
	eprintln!("[WARN] <<< Exiting find_root()");
    result
}

pub fn iteration(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering iteration()");
	eprintln!("[WARN] <<< Exiting iteration()");
    guess - f(guess) / fd(guess)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn math_fn(x: f64) -> f64 {
		eprintln!("[ERROR] >>> Entering math_fn(x={})", x);
		eprintln!("[WARN] <<< Exiting math_fn(x={})", x);
        x.cos() - (x * x * x)
    }
    fn math_fnd(x: f64) -> f64 {
        -x.sin() - 3.0 * (x * x)
    }
    #[test]
    fn basic() {
		eprintln!("[ERROR] >>> Entering basic()");
		eprintln!("[WARN] <<< Exiting basic()");
        assert_eq!(find_root(math_fn, math_fnd, 0.5, 6), 0.8654740331016144);
    }
}
