use std::cmp::{Ord, Ordering};

use rand::RngExt;

fn _quick_sort_3_ways<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
	eprintln!("[ERROR] >>> Entering _quick_sort_3_ways(lo={},hi={})", lo, hi);
    if lo >= hi {
        return;
    }

    let mut rng = rand::rng();
    arr.swap(lo, rng.random_range(lo..=hi));

    let mut lt = lo; // arr[lo+1, lt] < v
    let mut gt = hi + 1; // arr[gt, r] > v
    let mut i = lo + 1; // arr[lt + 1, i) == v

    while i < gt {
        match arr[i].cmp(&arr[lo]) {
            Ordering::Less => {
                arr.swap(i, lt + 1);
                i += 1;
                lt += 1;
            }
            Ordering::Greater => {
                arr.swap(i, gt - 1);
                gt -= 1;
            }
            Ordering::Equal => {
                i += 1;
            }
        }
    }

    arr.swap(lo, lt);

    if lt > 1 {
        _quick_sort_3_ways(arr, lo, lt - 1);
    }

	eprintln!("[WARN] <<< Exiting _quick_sort_3_ways(lo={},hi={})", lo, hi);
    _quick_sort_3_ways(arr, gt, hi);
}

pub fn quick_sort_3_ways<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering quick_sort_3_ways()");
    let len = arr.len();
    if len > 1 {
		eprintln!("[ERROR] quick_sort_3_ways(): len → {}", len);
		eprintln!("[WARN] <<< Exiting quick_sort_3_ways()");
        _quick_sort_3_ways(arr, 0, len - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;
    use crate::sorting::sort_utils;

    #[test]
    fn basic() {
		eprintln!("[ERROR] >>> Entering basic()");
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        sort_utils::log_timed("basic", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting basic()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
		eprintln!("[ERROR] >>> Entering basic_string()");
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        sort_utils::log_timed("basic string", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting basic_string()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        sort_utils::log_timed("empty", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting empty()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
		eprintln!("[ERROR] >>> Entering one_element()");
        let mut res = sort_utils::generate_random_vec(1, 0, 1);
        let cloned = res.clone();
        sort_utils::log_timed("one element", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting one_element()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
		eprintln!("[ERROR] >>> Entering pre_sorted()");
        let mut res = sort_utils::generate_nearly_ordered_vec(300000, 0);
        let cloned = res.clone();
        sort_utils::log_timed("pre sorted", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting pre_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
		eprintln!("[ERROR] >>> Entering reverse_sorted()");
        let mut res = sort_utils::generate_reverse_ordered_vec(300000);
        let cloned = res.clone();
        sort_utils::log_timed("reverse sorted", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting reverse_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
		eprintln!("[ERROR] >>> Entering large_elements()");
        let mut res = sort_utils::generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        sort_utils::log_timed("large elements test", || {
            quick_sort_3_ways(&mut res);
        });
		eprintln!("[WARN] <<< Exiting large_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
		eprintln!("[ERROR] >>> Entering nearly_ordered_elements()");
        let mut res = sort_utils::generate_nearly_ordered_vec(300000, 10);
        let cloned = res.clone();

        sort_utils::log_timed("nearly ordered elements test", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting nearly_ordered_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
		eprintln!("[ERROR] >>> Entering repeated_elements()");
        let mut res = sort_utils::generate_repeated_elements_vec(1000000, 3);
        let cloned = res.clone();

        sort_utils::log_timed("repeated elements test", || {
            quick_sort_3_ways(&mut res);
        });

		eprintln!("[WARN] <<< Exiting repeated_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
