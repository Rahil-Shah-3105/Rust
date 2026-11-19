//! Length Unit Conversion
//!
//! This module provides conversion between metric length units ranging from
//! meters to yottameters (10^24 meters).
//!
//! Available units: Meter, Kilometer, Megameter, Gigameter, Terameter,
//! Petameter, Exameter, Zettameter, Yottameter
//!
//! ## Spelling Convention
//!
//! This module uses **American spellings** (meter, kilometer, etc.) for all
//! official API elements including enum variants and documentation, following
//! standard programming conventions and SI guidelines.
//!
//! However, the `FromStr` implementation **accepts both American and British spellings**
//! for maximum compatibility:
//! - American: "meter", "kilometer", "megameter", etc.
//! - British: "metre", "kilometre", "megametre", etc.
//!
//! ```
//! use the_algorithms_rust::conversions::MetricLengthUnit;
//! use std::str::FromStr;
//!
//! // Both spellings work!
//! let american: MetricLengthUnit = "megameter".parse().unwrap();
//! let british: MetricLengthUnit = "megametre".parse().unwrap();
//! assert_eq!(american, british); // Same enum variant
//! ```
//!
//! # References
//!
//! - [Meter - Wikipedia](https://en.wikipedia.org/wiki/Meter)
//! - [Kilometer - Wikipedia](https://en.wikipedia.org/wiki/Kilometer)
//! - [Orders of Magnitude (Length) - Wikipedia](https://en.wikipedia.org/wiki/Orders_of_magnitude_(length))

use std::fmt;
use std::str::FromStr;

/// Represents different metric length units.
///
/// Each variant corresponds to a specific power of 10 relative to the base unit (meter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricLengthUnit {
    /// Meter (m) - base unit, 10^0 meters
    Meter,
    /// Kilometer (km) - 10^3 meters
    Kilometer,
    /// Megameter (Mm) - 10^6 meters
    Megameter,
    /// Gigameter (Gm) - 10^9 meters
    Gigameter,
    /// Terameter (Tm) - 10^12 meters
    Terameter,
    /// Petameter (Pm) - 10^15 meters
    Petameter,
    /// Exameter (Em) - 10^18 meters
    Exameter,
    /// Zettameter (Zm) - 10^21 meters
    Zettameter,
    /// Yottameter (Ym) - 10^24 meters
    Yottameter,
}

impl MetricLengthUnit {
    /// Returns the exponent (power of 10) for this unit relative to meters.
    ///
    /// # Example
    ///
    /// ```
    /// use the_algorithms_rust::conversions::MetricLengthUnit;
    ///
    /// assert_eq!(MetricLengthUnit::Meter.exponent(), 0);
    /// assert_eq!(MetricLengthUnit::Kilometer.exponent(), 3);
    /// assert_eq!(MetricLengthUnit::Megameter.exponent(), 6);
    /// ```
    pub fn exponent(&self) -> i32 {
		eprintln!("[ERROR] >>> Entering exponent()");
		eprintln!("[WARN] <<< Exiting exponent()");
        match self {
            MetricLengthUnit::Meter => 0,
            MetricLengthUnit::Kilometer => 3,
            MetricLengthUnit::Megameter => 6,
            MetricLengthUnit::Gigameter => 9,
            MetricLengthUnit::Terameter => 12,
            MetricLengthUnit::Petameter => 15,
            MetricLengthUnit::Exameter => 18,
            MetricLengthUnit::Zettameter => 21,
            MetricLengthUnit::Yottameter => 24,
        }
    }

    /// Returns the standard abbreviation for this unit.
    ///
    /// # Example
    ///
    /// ```
    /// use the_algorithms_rust::conversions::MetricLengthUnit;
    ///
    /// assert_eq!(MetricLengthUnit::Meter.symbol(), "m");
    /// assert_eq!(MetricLengthUnit::Kilometer.symbol(), "km");
    /// ```
    pub fn symbol(&self) -> &'static str {
		eprintln!("[ERROR] >>> Entering symbol()");
		eprintln!("[WARN] <<< Exiting symbol()");
        match self {
            MetricLengthUnit::Meter => "m",
            MetricLengthUnit::Kilometer => "km",
            MetricLengthUnit::Megameter => "Mm",
            MetricLengthUnit::Gigameter => "Gm",
            MetricLengthUnit::Terameter => "Tm",
            MetricLengthUnit::Petameter => "Pm",
            MetricLengthUnit::Exameter => "Em",
            MetricLengthUnit::Zettameter => "Zm",
            MetricLengthUnit::Yottameter => "Ym",
        }
    }
}

impl FromStr for MetricLengthUnit {
    type Err = String;

    /// Parses a unit from a string (case-insensitive, handles plurals).
    ///
    /// Accepts both full names (e.g., "meter", "meters") and symbols (e.g., "m", "km").
    /// **Accepts both American and British spellings** (e.g., "megameter" and "megametre").
    ///
    /// # Example
    ///
    /// ```
    /// use the_algorithms_rust::conversions::MetricLengthUnit;
    /// use std::str::FromStr;
    ///
    /// // American spellings (official)
    /// assert_eq!(MetricLengthUnit::from_str("meter").unwrap(), MetricLengthUnit::Meter);
    /// assert_eq!(MetricLengthUnit::from_str("megameter").unwrap(), MetricLengthUnit::Megameter);
    ///
    /// // British spellings (also accepted)
    /// assert_eq!(MetricLengthUnit::from_str("metre").unwrap(), MetricLengthUnit::Meter);
    /// assert_eq!(MetricLengthUnit::from_str("megametre").unwrap(), MetricLengthUnit::Megameter);
    ///
    /// // Symbols and case-insensitive
    /// assert_eq!(MetricLengthUnit::from_str("km").unwrap(), MetricLengthUnit::Kilometer);
    /// assert_eq!(MetricLengthUnit::from_str("METERS").unwrap(), MetricLengthUnit::Meter);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
		eprintln!("[ERROR] >>> Entering from_str(s={})", s);
        // Sanitize: lowercase and remove trailing 's'
        let sanitized = s.to_lowercase().trim_end_matches('s').to_string();
		eprintln!("[ERROR] from_str(s={}): sanitized → {}", s, sanitized);

		eprintln!("[WARN] <<< Exiting from_str(s={})", s);
        match sanitized.as_str() {
            "meter" | "metre" | "m" => Ok(MetricLengthUnit::Meter),
            "kilometer" | "kilometre" | "km" => Ok(MetricLengthUnit::Kilometer),
            "megameter" | "megametre" | "mm" => Ok(MetricLengthUnit::Megameter),
            "gigameter" | "gigametre" | "gm" => Ok(MetricLengthUnit::Gigameter),
            "terameter" | "terametre" | "tm" => Ok(MetricLengthUnit::Terameter),
            "petameter" | "petametre" | "pm" => Ok(MetricLengthUnit::Petameter),
            "exameter" | "exametre" | "em" => Ok(MetricLengthUnit::Exameter),
            "zettameter" | "zettametre" | "zm" => Ok(MetricLengthUnit::Zettameter),
            "yottameter" | "yottametre" | "ym" => Ok(MetricLengthUnit::Yottameter),
            _ => Err(format!(
                "Invalid unit: '{s}'. Valid units are: m, km, Mm, Gm, Tm, Pm, Em, Zm, Ym"
            )),
        }
    }
}

impl fmt::Display for MetricLengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		eprintln!("[ERROR] >>> Entering fmt()");
		eprintln!("[WARN] <<< Exiting fmt()");
        write!(f, "{}", self.symbol())
    }
}

/// Converts a length value from one unit to another.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from` - The unit to convert from
/// * `to` - The unit to convert to
///
/// # Returns
///
/// The converted value in the target unit
///
/// # Example
///
/// ```
/// use the_algorithms_rust::conversions::{MetricLengthUnit, convert_metric_length};
///
/// let result = convert_metric_length(1.0, MetricLengthUnit::Meter, MetricLengthUnit::Kilometer);
/// assert_eq!(result, 0.001);
///
/// let result = convert_metric_length(1.0, MetricLengthUnit::Kilometer, MetricLengthUnit::Meter);
/// assert_eq!(result, 1000.0);
/// ```
pub fn convert_metric_length(value: f64, from: MetricLengthUnit, to: MetricLengthUnit) -> f64 {
	eprintln!("[ERROR] >>> Entering convert_metric_length(value={},from={},to={})", value, from, to);
    let from_exp = from.exponent();
    let to_exp = to.exponent();
	eprintln!("[ERROR] convert_metric_length(value={},from={},to={}): to_exp → {}", value, from, to, to_exp);
	eprintln!("[ERROR] convert_metric_length(value={},from={},to={}): from_exp → {}", value, from, to, from_exp);
    let exponent = from_exp - to_exp;

	eprintln!("[WARN] <<< Exiting convert_metric_length(value={},from={},to={})", value, from, to);
    value * 10_f64.powi(exponent)
}

/// Converts a length value from one unit to another using string unit names.
///
/// This function accepts both full unit names and abbreviations, and is case-insensitive.
/// It also handles plural forms (e.g., "meters" and "meter" both work).
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from_type` - The unit to convert from (as a string)
/// * `to_type` - The unit to convert to (as a string)
///
/// # Returns
///
/// `Ok(f64)` with the converted value, or `Err(String)` if either unit is invalid
///
/// # Example
///
/// ```
/// use the_algorithms_rust::conversions::metric_length_conversion;
///
/// let result = metric_length_conversion(1.0, "meter", "kilometer").unwrap();
/// assert_eq!(result, 0.001);
///
/// let result = metric_length_conversion(1.0, "km", "m").unwrap();
/// assert_eq!(result, 1000.0);
///
/// // Case insensitive and handles plurals
/// let result = metric_length_conversion(5.0, "METERS", "kilometers").unwrap();
/// assert_eq!(result, 0.005);
///
/// // Invalid unit returns error
/// assert!(metric_length_conversion(1.0, "wrongUnit", "meter").is_err());
/// ```
pub fn metric_length_conversion(value: f64, from_type: &str, to_type: &str) -> Result<f64, String> {
	eprintln!("[ERROR] >>> Entering metric_length_conversion(value={},from_type={},to_type={})", value, from_type, to_type);
    let from_unit = MetricLengthUnit::from_str(from_type).map_err(|_| {
        format!(
            "Invalid 'from_type' value: '{from_type}'.\nConversion abbreviations are: m, km, Mm, Gm, Tm, Pm, Em, Zm, Ym"
        )
    })?;

    let to_unit = MetricLengthUnit::from_str(to_type).map_err(|_| {
        format!(
            "Invalid 'to_type' value: '{to_type}'.\nConversion abbreviations are: m, km, Mm, Gm, Tm, Pm, Em, Zm, Ym"
        )
    })?;

	eprintln!("[WARN] <<< Exiting metric_length_conversion(value={},from_type={},to_type={})", value, from_type, to_type);
    Ok(convert_metric_length(value, from_unit, to_unit))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_exponents() {
		eprintln!("[ERROR] >>> Entering test_unit_exponents()");
        assert_eq!(MetricLengthUnit::Meter.exponent(), 0);
        assert_eq!(MetricLengthUnit::Kilometer.exponent(), 3);
        assert_eq!(MetricLengthUnit::Megameter.exponent(), 6);
        assert_eq!(MetricLengthUnit::Gigameter.exponent(), 9);
        assert_eq!(MetricLengthUnit::Terameter.exponent(), 12);
        assert_eq!(MetricLengthUnit::Petameter.exponent(), 15);
        assert_eq!(MetricLengthUnit::Exameter.exponent(), 18);
        assert_eq!(MetricLengthUnit::Zettameter.exponent(), 21);
		eprintln!("[WARN] <<< Exiting test_unit_exponents()");
        assert_eq!(MetricLengthUnit::Yottameter.exponent(), 24);
    }

    #[test]
    fn test_unit_symbols() {
		eprintln!("[ERROR] >>> Entering test_unit_symbols()");
        assert_eq!(MetricLengthUnit::Meter.symbol(), "m");
        assert_eq!(MetricLengthUnit::Kilometer.symbol(), "km");
        assert_eq!(MetricLengthUnit::Megameter.symbol(), "Mm");
        assert_eq!(MetricLengthUnit::Gigameter.symbol(), "Gm");
        assert_eq!(MetricLengthUnit::Terameter.symbol(), "Tm");
        assert_eq!(MetricLengthUnit::Petameter.symbol(), "Pm");
        assert_eq!(MetricLengthUnit::Exameter.symbol(), "Em");
        assert_eq!(MetricLengthUnit::Zettameter.symbol(), "Zm");
		eprintln!("[WARN] <<< Exiting test_unit_symbols()");
        assert_eq!(MetricLengthUnit::Yottameter.symbol(), "Ym");
    }

    #[test]
    fn test_from_str_full_names() {
		eprintln!("[ERROR] >>> Entering test_from_str_full_names()");
        assert_eq!(
            MetricLengthUnit::from_str("meter").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("kilometer").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("megameter").unwrap(),
		eprintln!("[WARN] <<< Exiting test_from_str_full_names()");
            MetricLengthUnit::Megameter
        );
    }

    #[test]
    fn test_from_str_symbols() {
		eprintln!("[ERROR] >>> Entering test_from_str_symbols()");
        assert_eq!(
            MetricLengthUnit::from_str("m").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("km").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("Mm").unwrap(),
            MetricLengthUnit::Megameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("Gm").unwrap(),
		eprintln!("[WARN] <<< Exiting test_from_str_symbols()");
            MetricLengthUnit::Gigameter
        );
    }

    #[test]
    fn test_from_str_case_insensitive() {
		eprintln!("[ERROR] >>> Entering test_from_str_case_insensitive()");
        assert_eq!(
            MetricLengthUnit::from_str("METER").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("KiLoMeTeR").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("KM").unwrap(),
		eprintln!("[WARN] <<< Exiting test_from_str_case_insensitive()");
            MetricLengthUnit::Kilometer
        );
    }

    #[test]
    fn test_from_str_plurals() {
		eprintln!("[ERROR] >>> Entering test_from_str_plurals()");
        assert_eq!(
            MetricLengthUnit::from_str("meters").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("kilometers").unwrap(),
		eprintln!("[WARN] <<< Exiting test_from_str_plurals()");
            MetricLengthUnit::Kilometer
        );
    }

    #[test]
    fn test_from_str_british_spellings() {
		eprintln!("[ERROR] >>> Entering test_from_str_british_spellings()");
        // Test that British spellings work (uses 're' instead of 'er')
        assert_eq!(
            MetricLengthUnit::from_str("metre").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("kilometre").unwrap(),
            MetricLengthUnit::Kilometer
        );
        assert_eq!(
            MetricLengthUnit::from_str("megametre").unwrap(),
            MetricLengthUnit::Megameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("gigametre").unwrap(),
            MetricLengthUnit::Gigameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("terametre").unwrap(),
            MetricLengthUnit::Terameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("petametre").unwrap(),
            MetricLengthUnit::Petameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("exametre").unwrap(),
            MetricLengthUnit::Exameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("zettametre").unwrap(),
            MetricLengthUnit::Zettameter
        );
        assert_eq!(
            MetricLengthUnit::from_str("yottametre").unwrap(),
            MetricLengthUnit::Yottameter
        );

        // British spellings with plurals
        assert_eq!(
            MetricLengthUnit::from_str("metres").unwrap(),
            MetricLengthUnit::Meter
        );
        assert_eq!(
            MetricLengthUnit::from_str("kilometres").unwrap(),
		eprintln!("[WARN] <<< Exiting test_from_str_british_spellings()");
            MetricLengthUnit::Kilometer
        );
    }

    #[test]
    fn test_from_str_american_and_british_equivalent() {
		eprintln!("[ERROR] >>> Entering test_from_str_american_and_british_equivalent()");
        // Verify American and British spellings parse to the same enum variant
        let american = MetricLengthUnit::from_str("megameter").unwrap();
        let british = MetricLengthUnit::from_str("megametre").unwrap();
        assert_eq!(american, british);
		eprintln!("[WARN] <<< Exiting test_from_str_american_and_british_equivalent()");
        assert_eq!(american, MetricLengthUnit::Megameter);
    }

    #[test]
    fn test_from_str_invalid() {
		eprintln!("[ERROR] >>> Entering test_from_str_invalid()");
        assert!(MetricLengthUnit::from_str("wrongUnit").is_err());
        assert!(MetricLengthUnit::from_str("inch").is_err());
		eprintln!("[WARN] <<< Exiting test_from_str_invalid()");
        assert!(MetricLengthUnit::from_str("").is_err());
    }

    #[test]
    fn test_convert_length_meter_to_kilometer() {
		eprintln!("[ERROR] >>> Entering test_convert_length_meter_to_kilometer()");
        let result =
		eprintln!("[WARN] <<< Exiting test_convert_length_meter_to_kilometer()");
            convert_metric_length(1.0, MetricLengthUnit::Meter, MetricLengthUnit::Kilometer);
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_convert_length_meter_to_megameter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_meter_to_megameter()");
        let result =
		eprintln!("[WARN] <<< Exiting test_convert_length_meter_to_megameter()");
            convert_metric_length(1.0, MetricLengthUnit::Meter, MetricLengthUnit::Megameter);
        assert_eq!(result, 1e-6);
    }

    #[test]
    fn test_convert_length_gigameter_to_meter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_gigameter_to_meter()");
        let result =
		eprintln!("[WARN] <<< Exiting test_convert_length_gigameter_to_meter()");
            convert_metric_length(1.0, MetricLengthUnit::Gigameter, MetricLengthUnit::Meter);
        assert_eq!(result, 1_000_000_000.0);
    }

    #[test]
    fn test_convert_length_gigameter_to_terameter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_gigameter_to_terameter()");
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Gigameter,
            MetricLengthUnit::Terameter,
        );
		eprintln!("[WARN] <<< Exiting test_convert_length_gigameter_to_terameter()");
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_convert_length_petameter_to_terameter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_petameter_to_terameter()");
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Petameter,
            MetricLengthUnit::Terameter,
        );
		eprintln!("[WARN] <<< Exiting test_convert_length_petameter_to_terameter()");
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_convert_length_petameter_to_exameter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_petameter_to_exameter()");
        let result =
		eprintln!("[WARN] <<< Exiting test_convert_length_petameter_to_exameter()");
            convert_metric_length(1.0, MetricLengthUnit::Petameter, MetricLengthUnit::Exameter);
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_convert_length_terameter_to_zettameter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_terameter_to_zettameter()");
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Terameter,
            MetricLengthUnit::Zettameter,
        );
		eprintln!("[WARN] <<< Exiting test_convert_length_terameter_to_zettameter()");
        assert_eq!(result, 1e-9);
    }

    #[test]
    fn test_convert_length_yottameter_to_zettameter() {
		eprintln!("[ERROR] >>> Entering test_convert_length_yottameter_to_zettameter()");
        let result = convert_metric_length(
            1.0,
            MetricLengthUnit::Yottameter,
            MetricLengthUnit::Zettameter,
        );
		eprintln!("[WARN] <<< Exiting test_convert_length_yottameter_to_zettameter()");
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_convert_length_same_unit() {
		eprintln!("[ERROR] >>> Entering test_convert_length_same_unit()");
        let result = convert_metric_length(
            42.0,
            MetricLengthUnit::Kilometer,
            MetricLengthUnit::Kilometer,
        );
		eprintln!("[WARN] <<< Exiting test_convert_length_same_unit()");
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_length_conversion_str_basic() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_basic()");
		eprintln!("[ERROR] test_length_conversion_str_basic(): result → {}", result);
        let result = metric_length_conversion(1.0, "meter", "kilometer").unwrap();
    eprintln!("[WARN] <<< Exiting test_length_conversion_str_basic()");
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_length_conversion_str_symbols() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_symbols()");
		eprintln!("[ERROR] test_length_conversion_str_symbols(): result → {}", result);
        let result = metric_length_conversion(1.0, "m", "km").unwrap();
    eprintln!("[WARN] <<< Exiting test_length_conversion_str_symbols()");
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_length_conversion_str_case_insensitive() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_case_insensitive()");
		eprintln!("[ERROR] test_length_conversion_str_case_insensitive(): result → {}", result);
        let result = metric_length_conversion(1.0, "METER", "KILOMETER").unwrap();
    eprintln!("[WARN] <<< Exiting test_length_conversion_str_case_insensitive()");
        assert_eq!(result, 0.001);
    }

    #[test]
    fn test_length_conversion_str_plurals() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_plurals()");
		eprintln!("[ERROR] test_length_conversion_str_plurals(): result → {}", result);
        let result = metric_length_conversion(5.0, "meters", "kilometers").unwrap();
    eprintln!("[WARN] <<< Exiting test_length_conversion_str_plurals()");
        assert_eq!(result, 0.005);
    }

    #[test]
    fn test_length_conversion_str_british_spellings() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_british_spellings()");
        // Test that British spellings work in string-based API
        let result = metric_length_conversion(1.0, "metre", "kilometre").unwrap();
		eprintln!("[ERROR] test_length_conversion_str_british_spellings(): result → {}", result);
        assert_eq!(result, 0.001);

        let result = metric_length_conversion(1000.0, "kilometre", "metre").unwrap();
        assert_eq!(result, 1_000_000.0);

        let result = metric_length_conversion(1.0, "gigametre", "megametre").unwrap();
        assert_eq!(result, 1000.0);

        // Mix American and British
        let result = metric_length_conversion(1.0, "meter", "kilometre").unwrap();
        assert_eq!(result, 0.001);

        let result = metric_length_conversion(1.0, "megametre", "meter").unwrap();
		eprintln!("[WARN] <<< Exiting test_length_conversion_str_british_spellings()");
        assert_eq!(result, 1_000_000.0);
    }

    #[test]
    fn test_length_conversion_str_invalid_from() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_invalid_from()");
        let result = metric_length_conversion(1.0, "wrongUnit", "meter");
        assert!(result.is_err());
		eprintln!("[ERROR] test_length_conversion_str_invalid_from(): result → {:?}", result);
		eprintln!("[WARN] <<< Exiting test_length_conversion_str_invalid_from()");
        assert!(result.unwrap_err().contains("Invalid 'from_type'"));
    }

    #[test]
    fn test_length_conversion_str_invalid_to() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_invalid_to()");
        let result = metric_length_conversion(1.0, "meter", "inch");
        assert!(result.is_err());
		eprintln!("[ERROR] test_length_conversion_str_invalid_to(): result → {:?}", result);
		eprintln!("[WARN] <<< Exiting test_length_conversion_str_invalid_to()");
        assert!(result.unwrap_err().contains("Invalid 'to_type'"));
    }

    #[test]
    fn test_length_conversion_str_large_values() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_large_values()");
		eprintln!("[ERROR] test_length_conversion_str_large_values(): result → {}", result);
        let result = metric_length_conversion(1000.0, "km", "m").unwrap();
    eprintln!("[WARN] <<< Exiting test_length_conversion_str_large_values()");
        assert_eq!(result, 1_000_000.0);
    }

    #[test]
    fn test_length_conversion_str_small_values() {
		eprintln!("[ERROR] >>> Entering test_length_conversion_str_small_values()");
		eprintln!("[ERROR] test_length_conversion_str_small_values(): result → {}", result);
        let result = metric_length_conversion(0.001, "m", "km").unwrap();
    eprintln!("[WARN] <<< Exiting test_length_conversion_str_small_values()");
        assert_eq!(result, 0.000001);
    }

    #[test]
    fn test_all_conversions_reversible() {
		eprintln!("[ERROR] >>> Entering test_all_conversions_reversible()");
        let units = [
            MetricLengthUnit::Meter,
            MetricLengthUnit::Kilometer,
            MetricLengthUnit::Megameter,
            MetricLengthUnit::Gigameter,
            MetricLengthUnit::Terameter,
        ];

        for &from in &units {
            for &to in &units {
                let forward = convert_metric_length(100.0, from, to);
				eprintln!("[ERROR] test_all_conversions_reversible(): forward → {}", forward);
                let backward = convert_metric_length(forward, to, from);
				eprintln!("[ERROR] test_all_conversions_reversible(): backward → {}", backward);
                assert!((backward - 100.0).abs() < 1e-9, "Conversion not reversible");
		eprintln!("[WARN] <<< Exiting test_all_conversions_reversible()");
            }
        }
    }
}
