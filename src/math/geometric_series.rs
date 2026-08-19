// Author : cyrixninja
// Wikipedia : https://en.wikipedia.org/wiki/Geometric_series
// Calculate a geometric series.

pub fn geometric_series(nth_term: f64, start_term_a: f64, common_ratio_r: f64) -> Vec<f64> {
	log::debug!(">>> Entering geometric_series(nth_term={},start_term_a={},common_ratio_r={})", nth_term, start_term_a, common_ratio_r);
    let mut series = Vec::new();
    let mut multiple = 1.0;

    for _ in 0..(nth_term as i32) {
        series.push(start_term_a * multiple);
        multiple *= common_ratio_r;
    }

	log::debug!("<<< Exiting geometric_series(nth_term={},start_term_a={},common_ratio_r={})", nth_term, start_term_a, common_ratio_r);
    series
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approx_eq(a: f64, b: f64) {
		log::debug!(">>> Entering assert_approx_eq(a={},b={})", a, b);
        let epsilon = 1e-10;
		log::debug!("<<< Exiting assert_approx_eq(a={},b={})", a, b);
        assert!((a - b).abs() < epsilon, "Expected {a}, found {b}");
    }

    #[test]
    fn test_geometric_series() {
		log::debug!(">>> Entering test_geometric_series()");
        let result = geometric_series(4.0, 2.0, 2.0);
        assert_eq!(result.len(), 4);
		log::debug!("test_geometric_series(): result → {:?}", result);
        assert_approx_eq(result[0], 2.0);
        assert_approx_eq(result[1], 4.0);
        assert_approx_eq(result[2], 8.0);
        assert_approx_eq(result[3], 16.0);

        let result = geometric_series(4.1, 2.1, 2.1);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], 2.1);
        assert_approx_eq(result[1], 4.41);
        assert_approx_eq(result[2], 9.261);
        assert_approx_eq(result[3], 19.4481);

        let result = geometric_series(4.0, -2.0, 2.0);
        assert_eq!(result.len(), 4);
        assert_approx_eq(result[0], -2.0);
        assert_approx_eq(result[1], -4.0);
        assert_approx_eq(result[2], -8.0);
		log::debug!("<<< Exiting test_geometric_series()");
        assert_approx_eq(result[3], -16.0);
    }
}
