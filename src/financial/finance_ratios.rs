// Calculating simple ratios like Return on Investment (ROI), Debt to Equity, Gross Profit Margin
// and Earnings per Sale (EPS)
pub fn return_on_investment(gain: f64, cost: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering return_on_investment(gain={},cost={})", gain, cost);
	eprintln!("[WARN] <<< Exiting return_on_investment(gain={},cost={})", gain, cost);
    (gain - cost) / cost
}

pub fn debt_to_equity(debt: f64, equity: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering debt_to_equity(debt={},equity={})", debt, equity);
	eprintln!("[WARN] <<< Exiting debt_to_equity(debt={},equity={})", debt, equity);
    debt / equity
}

pub fn gross_profit_margin(revenue: f64, cost: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering gross_profit_margin(revenue={},cost={})", revenue, cost);
	eprintln!("[WARN] <<< Exiting gross_profit_margin(revenue={},cost={})", revenue, cost);
    (revenue - cost) / revenue
}

pub fn earnings_per_sale(net_income: f64, pref_dividend: f64, share_avg: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering earnings_per_sale(net_income={},pref_dividend={},share_avg={})", net_income, pref_dividend, share_avg);
	eprintln!("[WARN] <<< Exiting earnings_per_sale(net_income={},pref_dividend={},share_avg={})", net_income, pref_dividend, share_avg);
    (net_income - pref_dividend) / share_avg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_on_investment() {
		eprintln!("[ERROR] >>> Entering test_return_on_investment()");
        // let gain = 1200, cost = 1000 thus, ROI = (1200 - 1000)/1000 = 0.2
        let result = return_on_investment(1200.0, 1000.0);
		eprintln!("[ERROR] test_return_on_investment(): result → {}", result);
		eprintln!("[WARN] <<< Exiting test_return_on_investment()");
        assert!((result - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_debt_to_equity() {
		eprintln!("[ERROR] >>> Entering test_debt_to_equity()");
        // let debt = 300, equity = 150 thus, debt to equity ratio = 300/150 = 2
        let result = debt_to_equity(300.0, 150.0);
		eprintln!("[ERROR] test_debt_to_equity(): result → {}", result);
		eprintln!("[WARN] <<< Exiting test_debt_to_equity()");
        assert!((result - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_gross_profit_margin() {
		eprintln!("[ERROR] >>> Entering test_gross_profit_margin()");
        // let revenue = 1000, cost = 800 thus, gross profit margin = (1000-800)/1000 = 0.2
        let result = gross_profit_margin(1000.0, 800.0);
		eprintln!("[ERROR] test_gross_profit_margin(): result → {}", result);
		eprintln!("[WARN] <<< Exiting test_gross_profit_margin()");
        assert!((result - 0.2).abs() < 0.01);
    }

    #[test]
    fn test_earnings_per_sale() {
		eprintln!("[ERROR] >>> Entering test_earnings_per_sale()");
        // let net_income = 350, pref_dividend = 50, share_avg = 25 this EPS = (350-50)/25 = 12
        let result = earnings_per_sale(350.0, 50.0, 25.0);
		eprintln!("[ERROR] test_earnings_per_sale(): result → {}", result);
		eprintln!("[WARN] <<< Exiting test_earnings_per_sale()");
        assert!((result - 12.0).abs() < 0.001);
    }
}
