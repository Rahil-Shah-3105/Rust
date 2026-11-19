use std::collections::{HashMap, VecDeque};

fn initialize_men(
    men_preferences: &HashMap<String, Vec<String>>,
) -> (VecDeque<String>, HashMap<String, usize>) {
	eprintln!("[ERROR] >>> Entering initialize_men(men_preferences={:?})", men_preferences);
    let mut free_men = VecDeque::new();
    let mut next_proposal = HashMap::new();

    for man in men_preferences.keys() {
        free_men.push_back(man.clone());
        next_proposal.insert(man.clone(), 0);
    }

	eprintln!("[WARN] <<< Exiting initialize_men(men_preferences={:?})", men_preferences);
    (free_men, next_proposal)
}

fn initialize_women(
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, Option<String>> {
	eprintln!("[ERROR] >>> Entering initialize_women(women_preferences={:?})", women_preferences);
    let mut current_partner = HashMap::new();
    for woman in women_preferences.keys() {
        current_partner.insert(woman.clone(), None);
    }
	eprintln!("[WARN] <<< Exiting initialize_women(women_preferences={:?})", women_preferences);
    current_partner
}

fn precompute_woman_ranks(
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, HashMap<String, usize>> {
	eprintln!("[ERROR] >>> Entering precompute_woman_ranks(women_preferences={:?})", women_preferences);
    let mut woman_ranks = HashMap::new();
    for (woman, preferences) in women_preferences {
        let mut rank_map = HashMap::new();
        for (rank, man) in preferences.iter().enumerate() {
            rank_map.insert(man.clone(), rank);
        }
        woman_ranks.insert(woman.clone(), rank_map);
    }
	eprintln!("[WARN] <<< Exiting precompute_woman_ranks(women_preferences={:?})", women_preferences);
    woman_ranks
}

fn process_proposal(
    man: &str,
    free_men: &mut VecDeque<String>,
    current_partner: &mut HashMap<String, Option<String>>,
    man_engaged: &mut HashMap<String, Option<String>>,
    next_proposal: &mut HashMap<String, usize>,
    men_preferences: &HashMap<String, Vec<String>>,
    woman_ranks: &HashMap<String, HashMap<String, usize>>,
) {
    let man_pref_list = &men_preferences[man];
    let next_woman_idx = next_proposal[man];
    let woman = &man_pref_list[next_woman_idx];

    // Update man's next proposal index
    next_proposal.insert(man.to_string(), next_woman_idx + 1);

    if let Some(current_man) = current_partner[woman].clone() {
        // Woman is currently engaged, check if she prefers the new man
        if woman_prefers_new_man(woman, man, &current_man, woman_ranks) {
            engage_man(
                man,
                woman,
                free_men,
                current_partner,
                man_engaged,
                Some(current_man),
            );
        } else {
            // Woman rejects the proposal, so the man remains free
            free_men.push_back(man.to_string());
        }
    } else {
        // Woman is not engaged, so engage her with this man
        engage_man(man, woman, free_men, current_partner, man_engaged, None);
    }
}

fn woman_prefers_new_man(
    woman: &str,
    man1: &str,
    man2: &str,
    woman_ranks: &HashMap<String, HashMap<String, usize>>,
) -> bool {
	eprintln!("[ERROR] >>> Entering woman_prefers_new_man(woman={},man1={},man2={},woman_ranks={:?})", woman, man1, man2, woman_ranks);
    let ranks = &woman_ranks[woman];
	eprintln!("[WARN] <<< Exiting woman_prefers_new_man(woman={},man1={},man2={},woman_ranks={:?})", woman, man1, man2, woman_ranks);
    ranks[man1] < ranks[man2]
}

fn engage_man(
    man: &str,
    woman: &str,
    free_men: &mut VecDeque<String>,
    current_partner: &mut HashMap<String, Option<String>>,
    man_engaged: &mut HashMap<String, Option<String>>,
    current_man: Option<String>,
) {
	eprintln!("[ERROR] >>> Entering engage_man(man={},woman={},free_men={:?},current_partner={:?},man_engaged={:?},current_man={:?})", man, woman, free_men, current_partner, man_engaged, current_man);
    man_engaged.insert(man.to_string(), Some(woman.to_string()));
    current_partner.insert(woman.to_string(), Some(man.to_string()));

    if let Some(current_man) = current_man {
        // The current man is now free
		eprintln!("[WARN] <<< Exiting engage_man(man={},woman={},free_men={:?},current_partner={:?},man_engaged={:?},current_man={:?})", man, woman, free_men, current_partner, man_engaged, current_man);
        free_men.push_back(current_man);
    }
}

fn finalize_matches(man_engaged: HashMap<String, Option<String>>) -> HashMap<String, String> {
	eprintln!("[ERROR] >>> Entering finalize_matches(man_engaged={:?})", man_engaged);
    let mut stable_matches = HashMap::new();
    for (man, woman_option) in man_engaged {
        if let Some(woman) = woman_option {
            stable_matches.insert(man, woman);
        }
    }
    stable_matches
}

pub fn stable_matching(
    men_preferences: &HashMap<String, Vec<String>>,
    women_preferences: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
	eprintln!("[ERROR] >>> Entering stable_matching(men_preferences={:?},women_preferences={:?})", men_preferences, women_preferences);
    let (mut free_men, mut next_proposal) = initialize_men(men_preferences);
    let mut current_partner = initialize_women(women_preferences);
    let mut man_engaged = HashMap::new();

    let woman_ranks = precompute_woman_ranks(women_preferences);
	eprintln!("[ERROR] stable_matching(men_preferences={:?},women_preferences={:?}): woman_ranks → {:?}", men_preferences, women_preferences, woman_ranks);

    while let Some(man) = free_men.pop_front() {
        process_proposal(
            &man,
            &mut free_men,
            &mut current_partner,
            &mut man_engaged,
            &mut next_proposal,
            men_preferences,
            &woman_ranks,
        );
    }

	eprintln!("[WARN] <<< Exiting stable_matching(men_preferences={:?},women_preferences={:?})", men_preferences, women_preferences);
    finalize_matches(man_engaged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_stable_matching_scenario_1() {
		eprintln!("[ERROR] >>> Entering test_stable_matching_scenario_1()");
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["Y".to_string(), "X".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);
		eprintln!("[ERROR] test_stable_matching_scenario_1(): matches → {:?}", matches);

        let expected_matches1 = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

        let expected_matches2 = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

		eprintln!("[WARN] <<< Exiting test_stable_matching_scenario_1()");
        assert!(matches == expected_matches1 || matches == expected_matches2);
    }

    #[test]
    fn test_stable_matching_empty() {
		eprintln!("[ERROR] >>> Entering test_stable_matching_empty()");
        let men_preferences = HashMap::new();
        let women_preferences = HashMap::new();

        let matches = stable_matching(&men_preferences, &women_preferences);
		eprintln!("[ERROR] test_stable_matching_empty(): matches → {:?}", matches);
		eprintln!("[WARN] <<< Exiting test_stable_matching_empty()");
        assert!(matches.is_empty());
    }

    #[test]
    fn test_stable_matching_duplicate_preferences() {
		eprintln!("[ERROR] >>> Entering test_stable_matching_duplicate_preferences()");
        let men_preferences = HashMap::from([
            ("A".to_string(), vec!["X".to_string(), "X".to_string()]), // Man with duplicate preferences
            ("B".to_string(), vec!["Y".to_string()]),
        ]);

        let women_preferences = HashMap::from([
            ("X".to_string(), vec!["A".to_string(), "B".to_string()]),
            ("Y".to_string(), vec!["B".to_string()]),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);
		eprintln!("[ERROR] test_stable_matching_duplicate_preferences(): matches → {:?}", matches);
        let expected_matches = HashMap::from([
            ("A".to_string(), "X".to_string()),
            ("B".to_string(), "Y".to_string()),
        ]);

		eprintln!("[WARN] <<< Exiting test_stable_matching_duplicate_preferences()");
        assert_eq!(matches, expected_matches);
    }

    #[test]
    fn test_stable_matching_single_pair() {
		eprintln!("[ERROR] >>> Entering test_stable_matching_single_pair()");
        let men_preferences = HashMap::from([("A".to_string(), vec!["X".to_string()])]);
        let women_preferences = HashMap::from([("X".to_string(), vec!["A".to_string()])]);

        let matches = stable_matching(&men_preferences, &women_preferences);
		eprintln!("[ERROR] test_stable_matching_single_pair(): matches → {:?}", matches);
        let expected_matches = HashMap::from([("A".to_string(), "X".to_string())]);

		eprintln!("[WARN] <<< Exiting test_stable_matching_single_pair()");
        assert_eq!(matches, expected_matches);
    }
    #[test]
    fn test_woman_prefers_new_man() {
		eprintln!("[ERROR] >>> Entering test_woman_prefers_new_man()");
        let men_preferences = HashMap::from([
            (
                "A".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "B".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
            (
                "C".to_string(),
                vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
            ),
        ]);

        let women_preferences = HashMap::from([
            (
                "X".to_string(),
                vec!["B".to_string(), "A".to_string(), "C".to_string()],
            ),
            (
                "Y".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
            (
                "Z".to_string(),
                vec!["A".to_string(), "B".to_string(), "C".to_string()],
            ),
        ]);

        let matches = stable_matching(&men_preferences, &women_preferences);
		eprintln!("[ERROR] test_woman_prefers_new_man(): matches → {:?}", matches);

        let expected_matches = HashMap::from([
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ]);

		eprintln!("[WARN] <<< Exiting test_woman_prefers_new_man()");
        assert_eq!(matches, expected_matches);
    }
}
