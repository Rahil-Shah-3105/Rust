/// Decision Tree classifier using the ID3 algorithm with entropy-based splitting.
/// The tree recursively splits data based on the feature that provides the highest information gain.
/// Supports both categorical and continuous features through threshold-based splitting.

#[derive(Debug, Clone, PartialEq)]
enum TreeNode {
    Leaf {
        class_label: f64,
        samples: usize,
    },
    InternalNode {
        feature_index: usize,
        threshold: f64,
        left: Box<TreeNode>,
        right: Box<TreeNode>,
        samples: usize,
    },
}

/// Calculate entropy of a set of labels
fn calculate_entropy(labels: &[f64]) -> f64 {
	log::debug!(">>> Entering calculate_entropy(labels={:?})", labels);
    if labels.is_empty() {
        return 0.0;
    }

    let total = labels.len() as f64;
	log::debug!("calculate_entropy(labels={:?}): total → (unprintable)", labels);
    let mut unique_labels: Vec<f64> = Vec::new();
    let mut counts = Vec::new();
	log::debug!("calculate_entropy(labels={:?}): counts → (unprintable)", labels);

    for &label in labels {
        let mut found = false;
		log::debug!("calculate_entropy(labels={:?}): found → {}", labels, found);
        for (i, &existing_label) in unique_labels.iter().enumerate() {
            if (existing_label - label).abs() < 1e-10 {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique_labels.push(label);
            counts.push(1);
        }
    }

    let mut entropy = 0.0;
    for &count in &counts {
        let probability = count as f64 / total;
        if probability > 0.0 {
            entropy -= probability * probability.log2();
        }
    }

    log::debug!("<<< Exiting calculate_entropy(labels={:?})", labels);
    entropy
}

/// Find the best split for a feature
fn find_best_split(data: &[(Vec<f64>, f64)], feature_index: usize) -> Option<(f64, f64)> {
	log::debug!(">>> Entering find_best_split(data={:?},feature_index={})", data, feature_index);
    if data.is_empty() {
        return None;
    }

    let num_samples = data.len();
	log::debug!("find_best_split(data={:?},feature_index={}): num_samples → {}", data, feature_index, num_samples);

    let mut feature_values: Vec<(f64, f64)> = data
        .iter()
        .map(|(features, label)| (features[feature_index], *label))
        .collect();

    feature_values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let parent_entropy =
        calculate_entropy(&data.iter().map(|(_, label)| *label).collect::<Vec<_>>());

    let mut best_threshold = feature_values[0].0;
	log::debug!("find_best_split(data={:?},feature_index={}): best_threshold → (unprintable)", data, feature_index);
    let mut best_gain = 0.0;
	log::debug!("find_best_split(data={:?},feature_index={}): best_gain → {}", data, feature_index, best_gain);

    for i in 1..num_samples {
        if feature_values[i].0 != feature_values[i - 1].0 {
            let threshold = f64::midpoint(feature_values[i].0, feature_values[i - 1].0);
			log::debug!("find_best_split(data={:?},feature_index={}): threshold → (unprintable)", data, feature_index);

            let left_labels: Vec<f64> = feature_values[..i]
                .iter()
                .map(|(_, label)| *label)
                .collect();
            let right_labels: Vec<f64> = feature_values[i..]
                .iter()
                .map(|(_, label)| *label)
                .collect();

            let left_entropy = calculate_entropy(&left_labels);
			log::debug!("find_best_split(data={:?},feature_index={}): left_entropy → {}", data, feature_index, left_entropy);
            let right_entropy = calculate_entropy(&right_labels);
			log::debug!("find_best_split(data={:?},feature_index={}): right_entropy → {}", data, feature_index, right_entropy);

            let left_weight = i as f64 / num_samples as f64;
            let right_weight = (num_samples - i) as f64 / num_samples as f64;
			log::debug!("find_best_split(data={:?},feature_index={}): right_weight → (unprintable)", data, feature_index);

            let weighted_entropy = left_weight * left_entropy + right_weight * right_entropy;
            let information_gain = parent_entropy - weighted_entropy;

            if information_gain > best_gain {
                best_gain = information_gain;
                best_threshold = threshold;
            }
        }
    }

    if best_gain > 0.0 {
        Some((best_threshold, best_gain))
    } else {
        None
	log::debug!("<<< Exiting find_best_split(data={:?},feature_index={})", data, feature_index);
    }
}

/// Find the best feature and threshold to split on
fn find_best_split_feature(
    data: &[(Vec<f64>, f64)],
    feature_indices: &[usize],
) -> Option<(usize, f64)> {
	log::debug!(">>> Entering find_best_split_feature(data={:?},feature_indices={:?})", data, feature_indices);
    if data.is_empty() || feature_indices.is_empty() {
        return None;
    }

    let mut best_feature_index = 0;
	log::debug!("find_best_split_feature(data={:?},feature_indices={:?}): best_feature_index → {}", data, feature_indices, best_feature_index);
    let mut best_threshold = 0.0;
	log::debug!("find_best_split_feature(data={:?},feature_indices={:?}): best_threshold → {}", data, feature_indices, best_threshold);
    let mut best_gain = 0.0;
	log::debug!("find_best_split_feature(data={:?},feature_indices={:?}): best_gain → {}", data, feature_indices, best_gain);

    for &feature_index in feature_indices {
        if let Some((threshold, gain)) = find_best_split(data, feature_index) {
            if gain > best_gain {
                best_gain = gain;
                best_threshold = threshold;
                best_feature_index = feature_index;
            }
        }
    }

    if best_gain > 0.0 {
        Some((best_feature_index, best_threshold))
    } else {
		log::debug!("<<< Exiting find_best_split_feature(data={:?},feature_indices={:?})", data, feature_indices);
        None
    }
}

/// Get the majority class label
fn get_majority_class(labels: &[f64]) -> f64 {
	log::debug!(">>> Entering get_majority_class(labels={:?})", labels);
    if labels.is_empty() {
        return 0.0;
    }

    let mut unique_labels: Vec<f64> = Vec::new();
    let mut counts = Vec::new();
	log::debug!("get_majority_class(labels={:?}): counts → (unprintable)", labels);

    for &label in labels {
        let mut found = false;
		log::debug!("get_majority_class(labels={:?}): found → {}", labels, found);
        for (i, &existing_label) in unique_labels.iter().enumerate() {
            if (existing_label - label).abs() < 1e-10 {
                counts[i] += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique_labels.push(label);
            counts.push(1);
        }
    }

    let mut max_index = 0;
	log::debug!("get_majority_class(labels={:?}): max_index → {}", labels, max_index);
    let mut max_count = 0;
	log::debug!("get_majority_class(labels={:?}): max_count → {}", labels, max_count);
    for (i, &count) in counts.iter().enumerate() {
        if count > max_count {
            max_count = count;
            max_index = i;
        }
    }

    log::debug!("<<< Exiting get_majority_class(labels={:?})", labels);
    unique_labels[max_index]
}

/// Build the decision tree recursively
fn build_tree(
    data: &[(Vec<f64>, f64)],
    feature_indices: &[usize],
    max_depth: usize,
    min_samples_split: usize,
	log::debug!(">>> Entering build_tree(data={:?},feature_indices={:?},max_depth={},min_samples_split={},current_depth={})", data, feature_indices, max_depth, min_samples_split, current_depth);
	log::debug!("<<< Exiting build_tree(data={:?},feature_indices={:?},max_depth={},min_samples_split={},current_depth={})", data, feature_indices, max_depth, min_samples_split, current_depth);
    current_depth: usize,
) -> TreeNode {
    let labels: Vec<f64> = data.iter().map(|(_, label)| *label).collect();

    // Count unique labels
    let mut unique_count = 0;
    for i in 0..labels.len() {
        let mut is_unique = true;
		log::debug!("build_tree(data={:?},feature_indices={:?},max_depth={},min_samples_split={},current_depth={}): is_unique → {}", data, feature_indices, max_depth, min_samples_split, current_depth, is_unique);
        for j in 0..i {
            if (labels[i] - labels[j]).abs() < 1e-10 {
                is_unique = false;
                break;
            }
        }
        if is_unique {
            unique_count += 1;
        }
    }

    if unique_count == 1
        || data.len() < min_samples_split
        || current_depth >= max_depth
        || feature_indices.is_empty()
    {
        let class_label = get_majority_class(&labels);
		log::debug!("build_tree(data={:?},feature_indices={:?},max_depth={},min_samples_split={},current_depth={}): class_label → {}", data, feature_indices, max_depth, min_samples_split, current_depth, class_label);
        return TreeNode::Leaf {
            class_label,
            samples: data.len(),
        };
    }

    if let Some((feature_index, threshold)) = find_best_split_feature(data, feature_indices) {
        let mut left_data = Vec::new();
        let mut right_data = Vec::new();

        for (features, label) in data {
            if features[feature_index] < threshold {
                left_data.push((features.clone(), *label));
            } else {
                right_data.push((features.clone(), *label));
            }
        }

        if left_data.is_empty() || right_data.is_empty() {
            let class_label = get_majority_class(&labels);
            return TreeNode::Leaf {
                class_label,
                samples: data.len(),
            };
        }

        let left_child = build_tree(
            &left_data,
            feature_indices,
            max_depth,
            min_samples_split,
            current_depth + 1,
        );
        let right_child = build_tree(
            &right_data,
            feature_indices,
            max_depth,
            min_samples_split,
            current_depth + 1,
        );

        TreeNode::InternalNode {
            feature_index,
            threshold,
            left: Box::new(left_child),
            right: Box::new(right_child),
            samples: data.len(),
        }
    } else {
        let class_label = get_majority_class(&labels);
        TreeNode::Leaf {
            class_label,
            samples: data.len(),
        }
    }
}

/// Predict the class label for a single test point
fn predict_tree(tree: &TreeNode, features: &[f64]) -> f64 {
	log::debug!(">>> Entering predict_tree(tree={:?},features={:?})", tree, features);
    match tree {
        TreeNode::Leaf { class_label, .. } => *class_label,
        TreeNode::InternalNode {
            feature_index,
            threshold,
            left,
            right,
            ..
        } => {
            if features[*feature_index] < *threshold {
                predict_tree(left, features)
            } else {
				log::debug!("<<< Exiting predict_tree(tree={:?},features={:?})", tree, features);
                predict_tree(right, features)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DecisionTree {
    tree: TreeNode,
}

impl DecisionTree {
    pub fn fit(
        training_data: Vec<(Vec<f64>, f64)>,
        max_depth: usize,
        min_samples_split: usize,
    ) -> Option<Self> {
		log::debug!(">>> Entering fit(training_data={:?},max_depth={},min_samples_split={})", training_data, max_depth, min_samples_split);
        if training_data.is_empty() {
            return None;
        }

        let num_features = training_data[0].0.len();
		log::debug!("fit(training_data={:?},max_depth={},min_samples_split={}): num_features → {}", training_data, max_depth, min_samples_split, num_features);
        if num_features == 0 {
            return None;
        }

        let feature_indices: Vec<usize> = (0..num_features).collect();
        let tree = build_tree(
            &training_data,
            &feature_indices,
            max_depth,
            min_samples_split,
            0,
        );

        log::debug!("<<< Exiting fit(training_data={:?},max_depth={},min_samples_split={})", training_data, max_depth, min_samples_split);
        Some(DecisionTree { tree })
    }

    pub fn predict(&self, test_point: &[f64]) -> Option<f64> {
		log::debug!(">>> Entering predict(test_point={:?})", test_point);
        if test_point.is_empty() {
            return None;
        }

        Some(predict_tree(&self.tree, test_point))
    }

    #[allow(dead_code)]
    pub fn predict_batch(&self, test_points: &[Vec<f64>]) -> Vec<Option<f64>> {
		log::debug!(">>> Entering predict_batch(test_points={:?})", test_points);
        log::debug!("<<< Exiting predict_batch(test_points={:?})", test_points);
        test_points
            .iter()
            .map(|point| self.predict(point))
            .collect()
    }
}

/// Convenience function to train a decision tree and make predictions
pub fn decision_tree(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    max_depth: usize,
    min_samples_split: usize,
) -> Option<f64> {
	log::debug!(">>> Entering decision_tree(training_data={:?},test_point={:?},max_depth={},min_samples_split={})", training_data, test_point, max_depth, min_samples_split);
    let model = DecisionTree::fit(training_data, max_depth, min_samples_split)?;
	log::debug!("decision_tree(training_data={:?},test_point={:?},max_depth={},min_samples_split={}): model → (unprintable)", training_data, test_point, max_depth, min_samples_split);
    log::debug!("<<< Exiting decision_tree(training_data={:?},test_point={:?},max_depth={},min_samples_split={})", training_data, test_point, max_depth, min_samples_split);
    model.predict(&test_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_tree_simple_xor() {
		log::debug!(">>> Entering test_decision_tree_simple_xor()");
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_simple_xor(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();

        // XOR is difficult for decision trees with small dataset
        // Just verify the model can make predictions (not necessarily perfect for XOR)
        let result = model.predict(&[0.0, 0.0]);
		log::debug!("test_decision_tree_simple_xor(): result → {:?}", result);
        assert!(result.is_some());

        let result = model.predict(&[1.0, 1.0]);
		log::debug!("<<< Exiting test_decision_tree_simple_xor()");
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_linearly_separable() {
		log::debug!(">>> Entering test_decision_tree_linearly_separable()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![3.0, 3.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![7.0, 7.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_linearly_separable(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
		log::debug!("<<< Exiting test_decision_tree_linearly_separable()");
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
    }

    #[test]
    fn test_decision_tree_one_feature() {
		log::debug!(">>> Entering test_decision_tree_one_feature()");
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![5.0], 1.0),
            (vec![6.0], 1.0),
            (vec![7.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_one_feature(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[2.5]), Some(0.0));
		log::debug!("<<< Exiting test_decision_tree_one_feature()");
        assert_eq!(model.predict(&[5.5]), Some(1.0));
    }

    #[test]
    fn test_decision_tree_multiple_classes() {
		log::debug!(">>> Entering test_decision_tree_multiple_classes()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![10.0, 10.0], 2.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_multiple_classes(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
		log::debug!("<<< Exiting test_decision_tree_multiple_classes()");
        assert_eq!(model.predict(&[9.5, 9.5]), Some(2.0));
    }

    #[test]
    fn test_decision_tree_empty_training_data() {
		log::debug!(">>> Entering test_decision_tree_empty_training_data()");
        let training_data = vec![];
        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_empty_training_data(): model → (unprintable)");
		log::debug!("test_decision_tree_empty_training_data(): training_data → (unprintable)");
    log::debug!("<<< Exiting test_decision_tree_empty_training_data()");
        assert_eq!(model, None);
    }

    #[test]
    fn test_decision_tree_empty_features() {
		log::debug!(">>> Entering test_decision_tree_empty_features()");
        let training_data = vec![(vec![], 0.0), (vec![], 1.0)];
        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_empty_features(): model → (unprintable)");
		log::debug!("test_decision_tree_empty_features(): training_data → (unprintable)");
    log::debug!("<<< Exiting test_decision_tree_empty_features()");
        assert_eq!(model, None);
    }

    #[test]
    fn test_decision_tree_max_depth() {
		log::debug!(">>> Entering test_decision_tree_max_depth()");
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
        ];

        let model = DecisionTree::fit(training_data, 1, 2);
		log::debug!("test_decision_tree_max_depth(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();
        let result = model.predict(&[0.5, 0.5]);
		log::debug!("test_decision_tree_max_depth(): result → {:?}", result);
		log::debug!("<<< Exiting test_decision_tree_max_depth()");
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_min_samples_split() {
		log::debug!(">>> Entering test_decision_tree_min_samples_split()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 10);
		log::debug!("test_decision_tree_min_samples_split(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();
        let result = model.predict(&[1.5, 1.5]);
		log::debug!("test_decision_tree_min_samples_split(): result → {:?}", result);
		log::debug!("<<< Exiting test_decision_tree_min_samples_split()");
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_predict_batch() {
		log::debug!(">>> Entering test_decision_tree_predict_batch()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
		log::debug!("test_decision_tree_predict_batch(): model → (unprintable)");
        assert!(model.is_some());

        let model = model.unwrap();

        let test_points = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = model.predict_batch(&test_points);
		log::debug!("test_decision_tree_predict_batch(): predictions → {:?}", predictions);

        assert_eq!(predictions.len(), 2);
        assert_eq!(predictions[0], Some(0.0));
		log::debug!("<<< Exiting test_decision_tree_predict_batch()");
        assert_eq!(predictions[1], Some(1.0));
    }

    #[test]
    fn test_decision_tree_convenience_function() {
		log::debug!(">>> Entering test_decision_tree_convenience_function()");
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = decision_tree(training_data, vec![1.5, 1.5], 10, 2);
		log::debug!("test_decision_tree_convenience_function(): result → {:?}", result);
        assert_eq!(result, Some(0.0));

        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = decision_tree(training_data, vec![5.5, 5.5], 10, 2);
		log::debug!("<<< Exiting test_decision_tree_convenience_function()");
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_calculate_entropy() {
		log::debug!(">>> Entering test_calculate_entropy()");
        let labels = vec![0.0, 0.0, 0.0, 0.0];
        let entropy = calculate_entropy(&labels);
		log::debug!("test_calculate_entropy(): entropy → {}", entropy);
        assert!((entropy - 0.0).abs() < 1e-10);

        let labels = vec![0.0, 0.0, 1.0, 1.0];
        let entropy = calculate_entropy(&labels);
        assert!((entropy - 1.0).abs() < 1e-10);

        let labels = vec![0.0, 1.0, 2.0];
        let entropy = calculate_entropy(&labels);
		log::debug!("<<< Exiting test_calculate_entropy()");
        assert!(entropy > 0.0 && entropy < 2.0);
    }

    #[test]
    fn test_get_majority_class() {
		log::debug!(">>> Entering test_get_majority_class()");
        let labels = vec![0.0, 0.0, 1.0, 1.0, 0.0];
        let majority = get_majority_class(&labels);
		log::debug!("test_get_majority_class(): majority → {}", majority);
        assert_eq!(majority, 0.0);

        let labels = vec![1.0, 1.0, 2.0, 2.0, 2.0];
        let majority = get_majority_class(&labels);
		log::debug!("<<< Exiting test_get_majority_class()");
        assert_eq!(majority, 2.0);
    }
}
