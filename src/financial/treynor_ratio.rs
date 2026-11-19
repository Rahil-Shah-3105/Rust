/// Calculates the Treynor Ratio for a portfolio.
///
/// # Inputs
/// - `portfolio_return`: Portfolio return
/// - `risk_free_rate`: Risk-free rate
/// - `beta`: Portfolio beta
/// where Beta is a financial metric that measures the systematic risk of a security or portfolio compared to the overall market.
///
/// # Output
/// - Returns excess return per unit of market risk
pub fn treynor_ratio(portfolio_return: f64, risk_free_rate: f64, beta: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering treynor_ratio(portfolio_return={},risk_free_rate={},beta={})", portfolio_return, risk_free_rate, beta);
    if beta == 0.0 {
        f64::NAN
    } else {
		eprintln!("[WARN] <<< Exiting treynor_ratio(portfolio_return={},risk_free_rate={},beta={})", portfolio_return, risk_free_rate, beta);
        (portfolio_return - risk_free_rate) / beta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treynor_ratio() {
		eprintln!("[ERROR] >>> Entering test_treynor_ratio()");
        // for portfolio_return = 0.10, risk_free_rate = 0.05, beta = 1.5
        // expected result: (0.10 - 0.05) / 1.5 = 0.033333...
		eprintln!("[WARN] <<< Exiting test_treynor_ratio()");
        assert!((treynor_ratio(0.10, 0.05, 1.50) - 0.03333).abs() < 0.01);
    }

    #[test]
    fn test_treynor_ratio_empty_beta() {
		eprintln!("[ERROR] >>> Entering test_treynor_ratio_empty_beta()");
        // test for zero beta (undefined ratio)
		eprintln!("[WARN] <<< Exiting test_treynor_ratio_empty_beta()");
        assert!(treynor_ratio(0.10, 0.05, 0.00).is_nan());
    }
}
