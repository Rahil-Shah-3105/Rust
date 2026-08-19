/// fast_power returns the result of base^power mod modulus
pub fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
	log::debug!(">>> Entering fast_power(base={},power={},modulus={})", base, power, modulus);
    assert!(base >= 1);

    let mut res = 1;
	log::debug!("fast_power(base={},power={},modulus={}): res → {}", base, power, modulus, res);
    while power > 0 {
        if power & 1 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
		log::debug!("fast_power(base={},power={},modulus={}): base → {}", base, power, modulus, base);
        power >>= 1;
    }
	log::debug!("<<< Exiting fast_power(base={},power={},modulus={})", base, power, modulus);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
		log::debug!(">>> Entering test()");
        const MOD: usize = 1000000007;
        assert_eq!(fast_power(2, 1, MOD), 2);
        assert_eq!(fast_power(2, 2, MOD), 4);
        assert_eq!(fast_power(2, 4, MOD), 16);
        assert_eq!(fast_power(3, 4, MOD), 81);
		log::debug!("<<< Exiting test()");
        assert_eq!(fast_power(2, 100, MOD), 976371285);
    }
}
