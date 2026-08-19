use std::f64;

#[derive(Clone, Debug)]
pub struct YinResult {
    sample_rate: f64,
    best_lag: usize,
    cmndf: Vec<f64>,
}

impl YinResult {
    pub fn get_frequency(&self) -> f64 {
		log::debug!(">>> Entering get_frequency()");
        log::debug!("<<< Exiting get_frequency()");
        self.sample_rate / self.best_lag as f64
    }

    pub fn get_frequency_with_interpolation(&self) -> f64 {
		log::debug!(">>> Entering get_frequency_with_interpolation()");
        let best_lag_with_interpolation = parabolic_interpolation(self.best_lag, &self.cmndf);
		log::debug!("get_frequency_with_interpolation(): best_lag_with_interpolation → {}", best_lag_with_interpolation);
        log::debug!("<<< Exiting get_frequency_with_interpolation()");
        self.sample_rate / best_lag_with_interpolation
    }
}

fn parabolic_interpolation(lag: usize, cmndf: &[f64]) -> f64 {
	log::debug!(">>> Entering parabolic_interpolation(lag={},cmndf={:?})", lag, cmndf);
    let x0 = lag.saturating_sub(1); // max(0, lag-1)
    let x2 = usize::min(cmndf.len() - 1, lag + 1);
	log::debug!("parabolic_interpolation(lag={},cmndf={:?}): x2 → (unprintable)", lag, cmndf);
	log::debug!("parabolic_interpolation(lag={},cmndf={:?}): x0 → (unprintable)", lag, cmndf);
    let s0 = cmndf[x0];
    let s1 = cmndf[lag];
    let s2 = cmndf[x2];
    let denom = s0 - 2.0 * s1 + s2;
    if denom == 0.0 {
        return lag as f64;
    }
    let delta = (s0 - s2) / (2.0 * denom);
	log::debug!("parabolic_interpolation(lag={},cmndf={:?}): delta → (unprintable)", lag, cmndf);
    log::debug!("<<< Exiting parabolic_interpolation(lag={},cmndf={:?})", lag, cmndf);
    lag as f64 + delta
}

#[derive(Clone, Debug)]
pub struct Yin {
    threshold: f64,
    min_lag: usize,
    max_lag: usize,
    sample_rate: f64,
}

impl Yin {
    pub fn init(
        threshold: f64,
        min_expected_frequency: f64,
        max_expected_frequency: f64,
        sample_rate: f64,
    ) -> Yin {
        let min_lag = (sample_rate / max_expected_frequency) as usize;
        let max_lag = (sample_rate / min_expected_frequency) as usize;
        Yin {
            threshold,
            min_lag,
            max_lag,
            sample_rate,
		log::debug!("init(threshold={},min_expected_frequency={},max_expected_frequency={},sample_rate={}): max_lag → (unprintable)", threshold, min_expected_frequency, max_expected_frequency, sample_rate);
        log::debug!("<<< Exiting init(threshold={},min_expected_frequency={},max_expected_frequency={},sample_rate={})", threshold, min_expected_frequency, max_expected_frequency, sample_rate);
        }
    }

    pub fn yin(&self, frequencies: &[f64]) -> Result<YinResult, String> {
		log::debug!(">>> Entering yin(frequencies={:?})", frequencies);
        let df = difference_function_values(frequencies, self.max_lag);
        let cmndf = cumulative_mean_normalized_difference_function(&df, self.max_lag);
		log::debug!("yin(frequencies={:?}): cmndf → {:?}", frequencies, cmndf);
        let best_lag = find_cmndf_argmin(&cmndf, self.min_lag, self.max_lag, self.threshold);
        match best_lag {
            _ if best_lag == 0 => Err(format!(
                "Could not find lag value which minimizes CMNDF below the given threshold {}",
                self.threshold
            )),
            _ => Ok(YinResult {
                sample_rate: self.sample_rate,
                best_lag,
                cmndf,
            }),
		log::debug!("yin(frequencies={:?}): best_lag → {}", frequencies, best_lag);
        log::debug!("yin(frequencies={:?}): df → {:?}", frequencies, df);
        log::debug!("yin(frequencies={:?}): best_lag → {}", frequencies, best_lag);
        log::debug!("<<< Exiting yin(frequencies={:?})", frequencies);
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn difference_function_values(frequencies: &[f64], max_lag: usize) -> Vec<f64> {
	log::debug!(">>> Entering difference_function_values(frequencies={:?},max_lag={})", frequencies, max_lag);
    let mut df_list = vec![0.0; max_lag + 1];
    for lag in 1..=max_lag {
        df_list[lag] = difference_function(frequencies, lag);
    }
	log::debug!("<<< Exiting difference_function_values(frequencies={:?},max_lag={})", frequencies, max_lag);
    df_list
}

fn difference_function(f: &[f64], lag: usize) -> f64 {
	log::debug!(">>> Entering difference_function(f={:?},lag={})", f, lag);
    let mut sum = 0.0;
    let n = f.len();
	log::debug!("difference_function(f={:?},lag={}): n → {}", f, lag, n);
    for i in 0..(n - lag) {
        let diff = f[i] - f[i + lag];
        sum += diff * diff;
    }
	log::debug!("<<< Exiting difference_function(f={:?},lag={})", f, lag);
    sum
}

const EPSILON: f64 = 1e-10;
fn cumulative_mean_normalized_difference_function(df: &[f64], max_lag: usize) -> Vec<f64> {
	log::debug!(">>> Entering cumulative_mean_normalized_difference_function(df={:?},max_lag={})", df, max_lag);
    let mut cmndf = vec![0.0; max_lag + 1];
    cmndf[0] = 1.0;
    let mut sum = 0.0;
    for lag in 1..=max_lag {
        sum += df[lag];
        cmndf[lag] = lag as f64 * df[lag] / if sum == 0.0 { EPSILON } else { sum };
    }
	log::debug!("<<< Exiting cumulative_mean_normalized_difference_function(df={:?},max_lag={})", df, max_lag);
    cmndf
}

fn find_cmndf_argmin(cmndf: &[f64], min_lag: usize, max_lag: usize, threshold: f64) -> usize {
	log::debug!(">>> Entering find_cmndf_argmin(cmndf={:?},min_lag={},max_lag={},threshold={})", cmndf, min_lag, max_lag, threshold);
    let mut lag = min_lag;
    while lag <= max_lag {
        if cmndf[lag] < threshold {
            while lag < max_lag && cmndf[lag + 1] < cmndf[lag] {
                lag += 1;
            }
            return lag;
        }
        lag += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sine_wave(frequency: f64, sample_rate: f64, duration_secs: f64) -> Vec<f64> {
		log::debug!(">>> Entering generate_sine_wave(frequency={},sample_rate={},duration_secs={})", frequency, sample_rate, duration_secs);
        let total_samples = (sample_rate * duration_secs).round() as usize;
        let two_pi_f = 2.0 * std::f64::consts::PI * frequency;

        (0..total_samples)
            .map(|n| {
                let t = n as f64 / sample_rate;
				log::debug!("<<< Exiting generate_sine_wave(frequency={},sample_rate={},duration_secs={})", frequency, sample_rate, duration_secs);
                (two_pi_f * t).sin()
            })
            .collect()
    }

    fn diff_from_actual_frequency_smaller_than_threshold(
        result_frequency: f64,
        actual_frequency: f64,
        threshold: f64,
    ) -> bool {
        let result_diff_from_actual_freq = (result_frequency - actual_frequency).abs();
		log::debug!("diff_from_actual_frequency_smaller_than_threshold(result_frequency={},actual_frequency={},threshold={}): result_diff_from_actual_freq → (unprintable)", result_frequency, actual_frequency, threshold);
        log::debug!("<<< Exiting diff_from_actual_frequency_smaller_than_threshold(result_frequency={},actual_frequency={},threshold={})", result_frequency, actual_frequency, threshold);
        result_diff_from_actual_freq < threshold
    }

    fn interpolation_better_than_raw_result(result: YinResult, frequency: f64) -> bool {
		log::debug!(">>> Entering interpolation_better_than_raw_result(result={:?},frequency={})", result, frequency);
        let result_frequency = result.get_frequency();
        let refined_frequency = result.get_frequency_with_interpolation();
		log::debug!("interpolation_better_than_raw_result(result={:?},frequency={}): refined_frequency → {}", result, frequency, refined_frequency);
		log::debug!("interpolation_better_than_raw_result(result={:?},frequency={}): result_diff → (unprintable)", result, frequency);
        let result_diff = (result_frequency - frequency).abs();
        let refined_diff = (refined_frequency - frequency).abs();
		log::debug!("interpolation_better_than_raw_result(result={:?},frequency={}): refined_diff → (unprintable)", result, frequency);
        log::debug!("interpolation_better_than_raw_result(result={:?},frequency={}): result_frequency → {}", result, frequency, result_frequency);
        log::debug!("<<< Exiting interpolation_better_than_raw_result(result={:?},frequency={})", result, frequency);
        refined_diff < result_diff
    }

    #[test]
    fn test_simple_sine() {
		log::debug!(">>> Entering test_simple_sine()");
        let sample_rate = 1000.0;
        let frequency = 12.0;
        let seconds = 10.0;
        let signal = generate_sine_wave(frequency, sample_rate, seconds);
		log::debug!("test_simple_sine(): signal → {:?}", signal);

        let min_expected_frequency = 10.0;
        let max_expected_frequency = 100.0;

        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let result = yin.yin(signal.as_slice());
		log::debug!("test_simple_sine(): result → {:?}", result);
        assert!(result.is_ok());
        let yin_result = result.unwrap();
		log::debug!("test_simple_sine(): yin_result → (unprintable)");

        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency(),
            frequency,
            1.0
        ));
        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency_with_interpolation(),
            frequency,
            1.0,
        ));

		log::debug!("<<< Exiting test_simple_sine()");
        assert!(interpolation_better_than_raw_result(yin_result, frequency));
    }

    #[test]
    fn test_sine_frequency_range() {
		log::debug!(">>> Entering test_sine_frequency_range()");
        let sample_rate = 5000.0;
        for freq in 30..50 {
            let frequency = freq as f64;
            let seconds = 2.0;
            let signal = generate_sine_wave(frequency, sample_rate, seconds);
			log::debug!("test_sine_frequency_range(): signal → {:?}", signal);

            let min_expected_frequency = 5.0;
            let max_expected_frequency = 100.0;

            let yin = Yin::init(
                0.1,
                min_expected_frequency,
                max_expected_frequency,
                sample_rate,
            );
            let result = yin.yin(signal.as_slice());
			log::debug!("test_sine_frequency_range(): result → {:?}", result);
            assert!(result.is_ok());
            let yin_result = result.unwrap();
			log::debug!("test_sine_frequency_range(): yin_result → (unprintable)");

            if (sample_rate as i32 % freq) == 0 {
                assert_eq!(yin_result.get_frequency(), frequency);
            } else {
                assert!(diff_from_actual_frequency_smaller_than_threshold(
                    yin_result.get_frequency(),
                    frequency,
                    1.0
                ));
                assert!(diff_from_actual_frequency_smaller_than_threshold(
                    yin_result.get_frequency_with_interpolation(),
                    frequency,
                    1.0,
                ));

                assert!(interpolation_better_than_raw_result(yin_result, frequency));
            }
		log::debug!("<<< Exiting test_sine_frequency_range()");
        }
    }

    #[test]
    fn test_harmonic_sines() {
		log::debug!(">>> Entering test_harmonic_sines()");
        let sample_rate = 44100.0;
        let seconds = 2.0;
        let frequency_1 = 50.0; // Minimal/Fundamental frequency - this is what YIN should find
        let signal_1 = generate_sine_wave(frequency_1, sample_rate, seconds);
		log::debug!("test_harmonic_sines(): signal_1 → {:?}", signal_1);
        let frequency_2 = 150.0;
        let signal_2 = generate_sine_wave(frequency_2, sample_rate, seconds);
		log::debug!("test_harmonic_sines(): signal_2 → {:?}", signal_2);
        let frequency_3 = 300.0;
        let signal_3 = generate_sine_wave(frequency_3, sample_rate, seconds);
		log::debug!("test_harmonic_sines(): signal_3 → {:?}", signal_3);

        let min_expected_frequency = 10.0;
        let max_expected_frequency = 500.0;

        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let total_samples = (sample_rate * seconds).round() as usize;
		log::debug!("test_harmonic_sines(): total_samples → (unprintable)");
        let combined_signal: Vec<f64> = (0..total_samples)
            .map(|n| signal_1[n] + signal_2[n] + signal_3[n])
            .collect();

        let result = yin.yin(&combined_signal);
		log::debug!("test_harmonic_sines(): result → {:?}", result);
        assert!(result.is_ok());
        let yin_result = result.unwrap();
		log::debug!("test_harmonic_sines(): yin_result → (unprintable)");

        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency(),
            frequency_1,
            1.0
		log::debug!("<<< Exiting test_harmonic_sines()");
        ));
    }

    #[test]
    fn test_unharmonic_sines() {
		log::debug!(">>> Entering test_unharmonic_sines()");
        let sample_rate = 44100.0;
        let seconds = 2.0;
        let frequency_1 = 50.0;
        let signal_1 = generate_sine_wave(frequency_1, sample_rate, seconds);
		log::debug!("test_unharmonic_sines(): signal_1 → {:?}", signal_1);
        let frequency_2 = 66.0;
        let signal_2 = generate_sine_wave(frequency_2, sample_rate, seconds);
		log::debug!("test_unharmonic_sines(): signal_2 → {:?}", signal_2);
        let frequency_3 = 300.0;
        let signal_3 = generate_sine_wave(frequency_3, sample_rate, seconds);
		log::debug!("test_unharmonic_sines(): signal_3 → {:?}", signal_3);

        let min_expected_frequency = 10.0;
        let max_expected_frequency = 500.0;

        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let total_samples = (sample_rate * seconds).round() as usize;
		log::debug!("test_unharmonic_sines(): total_samples → (unprintable)");
        let combined_signal: Vec<f64> = (0..total_samples)
            .map(|n| signal_1[n] + signal_2[n] + signal_3[n])
            .collect();

        let result = yin.yin(&combined_signal);
		log::debug!("test_unharmonic_sines(): result → {:?}", result);
        assert!(result.is_ok());
        let yin_result = result.unwrap();
		log::debug!("test_unharmonic_sines(): yin_result → (unprintable)");

        let expected_frequency = (frequency_1 - frequency_2).abs();
		log::debug!("test_unharmonic_sines(): expected_frequency → (unprintable)");
        assert!(diff_from_actual_frequency_smaller_than_threshold(
            yin_result.get_frequency(),
            expected_frequency,
            1.0
        ));
        assert!(interpolation_better_than_raw_result(
            yin_result,
            expected_frequency
		log::debug!("<<< Exiting test_unharmonic_sines()");
        ));
    }

    #[test]
    fn test_err() {
		log::debug!(">>> Entering test_err()");
        let sample_rate = 2500.0;
        let seconds = 2.0;
        let frequency = 440.0;

        // Can't find frequency 440 between 500 and 700
        let min_expected_frequency = 500.0;
        let max_expected_frequency = 700.0;
        let yin = Yin::init(
            0.1,
            min_expected_frequency,
            max_expected_frequency,
            sample_rate,
        );

        let signal = generate_sine_wave(frequency, sample_rate, seconds);
		log::debug!("test_err(): signal → {:?}", signal);
        let result = yin.yin(&signal);
		log::debug!("test_err(): result → {:?}", result);
        assert!(result.is_err());

        let yin_with_suitable_frequency_range = Yin::init(
            0.1,
            min_expected_frequency - 100.0,
            max_expected_frequency,
            sample_rate,
        );
        let result = yin_with_suitable_frequency_range.yin(&signal);
		log::debug!("<<< Exiting test_err()");
        assert!(result.is_ok());
    }
}
