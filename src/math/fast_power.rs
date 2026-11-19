/// fast_power returns the result of base^power mod modulus
pub fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
	eprintln!("[ERROR] >>> Entering fast_power(base={},power={},modulus={})", base, power, modulus);
    assert!(base >= 1);

    let mut res = 1;
	eprintln!("[ERROR] fast_power(base={},power={},modulus={}): res → {}", base, power, modulus, res);
    while power > 0 {
        if power & 1 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
		eprintln!("[ERROR] fast_power(base={},power={},modulus={}): base → {}", base, power, modulus, base);
        power >>= 1;
    }
	eprintln!("[WARN] <<< Exiting fast_power(base={},power={},modulus={})", base, power, modulus);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		eprintln!("[ERROR] >>> Entering test()");
        const MOD: usize = 1000000007;
        assert_eq!(fast_power(2, 1, MOD), 2);
        assert_eq!(fast_power(2, 2, MOD), 4);
        assert_eq!(fast_power(2, 4, MOD), 16);
        assert_eq!(fast_power(3, 4, MOD), 81);
		eprintln!("[WARN] <<< Exiting test()");
        assert_eq!(fast_power(2, 100, MOD), 976371285);
    }
}
