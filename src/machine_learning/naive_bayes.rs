/// Naive Bayes classifier for classification tasks.
/// This implementation uses Gaussian Naive Bayes, which assumes that
/// features follow a normal (Gaussian) distribution.
/// The algorithm calculates class priors and feature statistics (mean and variance)
/// for each class, then uses Bayes' theorem to predict class probabilities.

pub struct ClassStatistics {
    pub class_label: f64,
    pub prior: f64,
    pub feature_means: Vec<f64>,
    pub feature_variances: Vec<f64>,
}

fn calculate_class_statistics(
    training_data: &[(Vec<f64>, f64)],
    class_label: f64,
    num_features: usize,
) -> Option<ClassStatistics> {
	log::debug!(">>> Entering calculate_class_statistics(training_data={:?},class_label={},num_features={})", training_data, class_label, num_features);
    let class_samples: Vec<&(Vec<f64>, f64)> = training_data
        .iter()
        .filter(|(_, label)| (*label - class_label).abs() < 1e-10)
        .collect();

    if class_samples.is_empty() {
        return None;
    }

    let prior = class_samples.len() as f64 / training_data.len() as f64;
	log::debug!("calculate_class_statistics(training_data={:?},class_label={},num_features={}): prior → (unprintable)", training_data, class_label, num_features);

    let mut feature_means = vec![0.0; num_features];
    let mut feature_variances = vec![0.0; num_features];

    // Calculate means
    for (features, _) in &class_samples {
        for (i, &feature) in features.iter().enumerate() {
            if i < num_features {
                feature_means[i] += feature;
            }
        }
    }

    let n = class_samples.len() as f64;
    for mean in &mut feature_means {
        *mean /= n;
    }

    // Calculate variances
    for (features, _) in &class_samples {
        for (i, &feature) in features.iter().enumerate() {
            if i < num_features {
                let diff = feature - feature_means[i];
                feature_variances[i] += diff * diff;
            }
        }
    }

    let epsilon = 1e-9;
    for variance in &mut feature_variances {
        *variance = (*variance / n).max(epsilon);
    }

	log::debug!("calculate_class_statistics(training_data={:?},class_label={},num_features={}): n → (unprintable)", training_data, class_label, num_features);
    log::debug!("<<< Exiting calculate_class_statistics(training_data={:?},class_label={},num_features={})", training_data, class_label, num_features);
    Some(ClassStatistics {
        class_label,
        prior,
        feature_means,
        feature_variances,
    })
}

fn gaussian_log_pdf(x: f64, mean: f64, variance: f64) -> f64 {
	log::debug!(">>> Entering gaussian_log_pdf(x={},mean={},variance={})", x, mean, variance);
    let diff = x - mean;
    let exponent_term = -(diff * diff) / (2.0 * variance);
    let log_coefficient = -0.5 * (2.0 * std::f64::consts::PI * variance).ln();
	log::debug!("gaussian_log_pdf(x={},mean={},variance={}): log_coefficient → (unprintable)", x, mean, variance);
    log::debug!("<<< Exiting gaussian_log_pdf(x={},mean={},variance={})", x, mean, variance);
    log_coefficient + exponent_term
}

pub fn train_naive_bayes(training_data: Vec<(Vec<f64>, f64)>) -> Option<Vec<ClassStatistics>> {
	log::debug!(">>> Entering train_naive_bayes(training_data={:?})", training_data);
    if training_data.is_empty() {
        return None;
    }

    let num_features = training_data[0].0.len();
	log::debug!("train_naive_bayes(training_data={:?}): num_features → {}", training_data, num_features);
    if num_features == 0 {
        return None;
    }

    // Verify all samples have the same number of features
    if !training_data
        .iter()
        .all(|(features, _)| features.len() == num_features)
    {
        return None;
    }

    // Get unique class labels
    let mut unique_classes = Vec::new();
	log::debug!("train_naive_bayes(training_data={:?}): unique_classes → (unprintable)", training_data);
    for (_, label) in &training_data {
        if !unique_classes
            .iter()
            .any(|&c: &f64| (c - *label).abs() < 1e-10)
        {
            unique_classes.push(*label);
        }
    }

    let mut class_stats = Vec::new();
	log::debug!("train_naive_bayes(training_data={:?}): class_stats → (unprintable)", training_data);

    for class_label in unique_classes {
        if let Some(mut stats) =
            calculate_class_statistics(&training_data, class_label, num_features)
        {
            stats.class_label = class_label;
            class_stats.push(stats);
        }
    }

    if class_stats.is_empty() {
	log::debug!("train_naive_bayes(training_data={:?}): stats.class_label → (unprintable)", training_data);
    log::debug!("<<< Exiting train_naive_bayes(training_data={:?})", training_data);
        return None;
    }

    Some(class_stats)
}

pub fn predict_naive_bayes(model: &[ClassStatistics], test_point: &[f64]) -> Option<f64> {
	log::debug!(">>> Entering predict_naive_bayes(model=(unprintable),test_point={:?})", test_point);
    if model.is_empty() || test_point.is_empty() {
        return None;
    }

    // Get number of features from the first class statistics
    let num_features = model[0].feature_means.len();
	log::debug!("predict_naive_bayes(model=(unprintable),test_point={:?}): num_features → {}", test_point, num_features);
    if test_point.len() != num_features {
        return None;
    }

    let mut best_class = None;
	log::debug!("predict_naive_bayes(model=(unprintable),test_point={:?}): best_class → (unprintable)", test_point);
	log::debug!("predict_naive_bayes(model=(unprintable),test_point={:?}): best_log_prob → (unprintable)", test_point);
    let mut best_log_prob = f64::NEG_INFINITY;

    for stats in model {
        // Calculate log probability to avoid underflow
        let mut log_prob = stats.prior.ln();
		log::debug!("predict_naive_bayes(model=(unprintable),test_point={:?}): log_prob → (unprintable)", test_point);

        for (i, &feature) in test_point.iter().enumerate() {
            if i < stats.feature_means.len() && i < stats.feature_variances.len() {
                // Use log PDF directly to avoid numerical underflow
                log_prob +=
                    gaussian_log_pdf(feature, stats.feature_means[i], stats.feature_variances[i]);
            }
        }

        if log_prob > best_log_prob {
            best_log_prob = log_prob;
            best_class = Some(stats.class_label);
        }
    }

    log::debug!("<<< Exiting predict_naive_bayes(model=(unprintable),test_point={:?})", test_point);
    best_class
}

pub fn naive_bayes(training_data: Vec<(Vec<f64>, f64)>, test_point: Vec<f64>) -> Option<f64> {
	log::debug!(">>> Entering naive_bayes(training_data={:?},test_point={:?})", training_data, test_point);
    let model = train_naive_bayes(training_data)?;
	log::debug!("naive_bayes(training_data={:?},test_point={:?}): model → (unprintable)", training_data, test_point);
    log::debug!("<<< Exiting naive_bayes(training_data={:?},test_point={:?})", training_data, test_point);
    predict_naive_bayes(&model, &test_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_bayes_simple_classification() {
		log::debug!(">>> Entering test_naive_bayes_simple_classification()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![1.1, 1.0], 0.0),
            (vec![1.0, 1.1], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![5.1, 5.0], 1.0),
            (vec![5.0, 5.1], 1.0),
        ];

        // Test point closer to class 0
        let test_point = vec![1.05, 1.05];
        let result = naive_bayes(training_data.clone(), test_point);
		log::debug!("test_naive_bayes_simple_classification(): result → {:?}", result);
        assert_eq!(result, Some(0.0));

        // Test point closer to class 1
        let test_point = vec![5.05, 5.05];
        let result = naive_bayes(training_data, test_point);
		log::debug!("<<< Exiting test_naive_bayes_simple_classification()");
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_naive_bayes_one_dimensional() {
		log::debug!(">>> Entering test_naive_bayes_one_dimensional()");
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![1.1], 0.0),
            (vec![1.2], 0.0),
            (vec![5.0], 1.0),
            (vec![5.1], 1.0),
            (vec![5.2], 1.0),
        ];

        let test_point = vec![1.15];
        let result = naive_bayes(training_data.clone(), test_point);
		log::debug!("test_naive_bayes_one_dimensional(): result → {:?}", result);
        assert_eq!(result, Some(0.0));

        let test_point = vec![5.15];
        let result = naive_bayes(training_data, test_point);
		log::debug!("<<< Exiting test_naive_bayes_one_dimensional()");
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_naive_bayes_empty_training_data() {
		log::debug!(">>> Entering test_naive_bayes_empty_training_data()");
        let training_data = vec![];
        let test_point = vec![1.0, 2.0];
		log::debug!("test_naive_bayes_empty_training_data(): training_data → (unprintable)");
        let result = naive_bayes(training_data, test_point);
		log::debug!("test_naive_bayes_empty_training_data(): result → {:?}", result);
		log::debug!("<<< Exiting test_naive_bayes_empty_training_data()");
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_empty_test_point() {
		log::debug!(">>> Entering test_naive_bayes_empty_test_point()");
        let training_data = vec![(vec![1.0, 2.0], 0.0)];
        let test_point = vec![];
		log::debug!("test_naive_bayes_empty_test_point(): training_data → (unprintable)");
        let result = naive_bayes(training_data, test_point);
		log::debug!("test_naive_bayes_empty_test_point(): result → {:?}", result);
		log::debug!("<<< Exiting test_naive_bayes_empty_test_point()");
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_dimension_mismatch() {
		log::debug!(">>> Entering test_naive_bayes_dimension_mismatch()");
        let training_data = vec![(vec![1.0, 2.0], 0.0), (vec![3.0, 4.0], 1.0)];
        let test_point = vec![1.0]; // Wrong dimension
		log::debug!("test_naive_bayes_dimension_mismatch(): training_data → (unprintable)");
        let result = naive_bayes(training_data, test_point);
		log::debug!("test_naive_bayes_dimension_mismatch(): result → {:?}", result);
		log::debug!("<<< Exiting test_naive_bayes_dimension_mismatch()");
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_inconsistent_feature_dimensions() {
		log::debug!(">>> Entering test_naive_bayes_inconsistent_feature_dimensions()");
        let training_data = vec![
            (vec![1.0, 2.0], 0.0),
            (vec![3.0], 1.0), // Different dimension
        ];
        let test_point = vec![1.0, 2.0];
        let result = naive_bayes(training_data, test_point);
		log::debug!("test_naive_bayes_inconsistent_feature_dimensions(): result → {:?}", result);
		log::debug!("<<< Exiting test_naive_bayes_inconsistent_feature_dimensions()");
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_multiple_classes() {
		log::debug!(">>> Entering test_naive_bayes_multiple_classes()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![1.1, 1.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![5.1, 5.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![9.1, 9.0], 2.0),
        ];

        let test_point = vec![1.05, 1.05];
        let result = naive_bayes(training_data.clone(), test_point);
		log::debug!("test_naive_bayes_multiple_classes(): result → {:?}", result);
        assert_eq!(result, Some(0.0));

        let test_point = vec![5.05, 5.05];
        let result = naive_bayes(training_data.clone(), test_point);
        assert_eq!(result, Some(1.0));

        let test_point = vec![9.05, 9.05];
        let result = naive_bayes(training_data, test_point);
		log::debug!("<<< Exiting test_naive_bayes_multiple_classes()");
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn test_train_and_predict_separately() {
		log::debug!(">>> Entering test_train_and_predict_separately()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![1.1, 1.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![5.1, 5.0], 1.0),
        ];

        let model = train_naive_bayes(training_data);
		log::debug!("test_train_and_predict_separately(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();
        assert_eq!(model.len(), 2);

        let test_point = vec![1.05, 1.05];
        let result = predict_naive_bayes(&model, &test_point);
		log::debug!("test_train_and_predict_separately(): result → {:?}", result);
		log::debug!("<<< Exiting test_train_and_predict_separately()");
        assert_eq!(result, Some(0.0));
    }
}
