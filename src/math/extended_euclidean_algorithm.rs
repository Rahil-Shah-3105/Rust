fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
	log::debug!(">>> Entering update_step(a={},old_a={},quotient={})", a, old_a, quotient);
    let temp = *a;
    *a = *old_a - quotient * temp;
	log::debug!("<<< Exiting update_step(a={},old_a={},quotient={})", a, old_a, quotient);
    *old_a = temp;
}

pub fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
	log::debug!(">>> Entering extended_euclidean_algorithm(a={},b={})", a, b);
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

	log::debug!("<<< Exiting extended_euclidean_algorithm(a={},b={})", a, b);
    (old_r, old_s, old_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
		log::debug!(">>> Entering basic()");
        assert_eq!(extended_euclidean_algorithm(101, 13), (1, 4, -31));
        assert_eq!(extended_euclidean_algorithm(123, 19), (1, -2, 13));
        assert_eq!(extended_euclidean_algorithm(25, 36), (1, 13, -9));
        assert_eq!(extended_euclidean_algorithm(69, 54), (3, -7, 9));
        assert_eq!(extended_euclidean_algorithm(55, 79), (1, 23, -16));
        assert_eq!(extended_euclidean_algorithm(33, 44), (11, -1, 1));
		log::debug!("<<< Exiting basic()");
        assert_eq!(extended_euclidean_algorithm(50, 70), (10, 3, -2));
    }
}
