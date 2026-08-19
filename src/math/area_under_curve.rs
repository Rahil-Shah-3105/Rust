pub fn area_under_curve(start: f64, end: f64, func: fn(f64) -> f64, step_count: usize) -> f64 {
	log::debug!(">>> Entering area_under_curve(start={},end={})", start, end);
    assert!(step_count > 0);

    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    }; //swap if bounds reversed

    let step_length: f64 = (end - start) / step_count as f64;
    let mut area = 0f64;
    let mut fx1 = func(start);
    let mut fx2: f64;

    for eval_point in (1..=step_count).map(|x| (x as f64 * step_length) + start) {
        fx2 = func(eval_point);
        area += (fx2 + fx1).abs() * step_length * 0.5;
        fx1 = fx2;
    }

	log::debug!("<<< Exiting area_under_curve(start={},end={})", start, end);
    area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_func() {
		log::debug!(">>> Entering test_linear_func()");
		log::debug!("<<< Exiting test_linear_func()");
        assert_eq!(area_under_curve(1f64, 2f64, |x| x, 10), 1.5000000000000002);
    }

    #[test]
    fn test_quadratic_func() {
		log::debug!(">>> Entering test_quadratic_func()");
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x * x, 1000),
            2.333333500000005
		log::debug!("<<< Exiting test_quadratic_func()");
        );
    }

    #[test]
    fn test_zero_length() {
		log::debug!(">>> Entering test_zero_length()");
		log::debug!("<<< Exiting test_zero_length()");
        assert_eq!(area_under_curve(0f64, 0f64, |x| x * x, 1000), 0.0);
    }

    #[test]
    fn test_reverse() {
		log::debug!(">>> Entering test_reverse()");
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x, 10),
            area_under_curve(2f64, 1f64, |x| x, 10)
		log::debug!("<<< Exiting test_reverse()");
        );
    }
}
