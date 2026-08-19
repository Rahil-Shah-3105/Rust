pub fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
	log::debug!(">>> Entering partition(lo={},hi={})", lo, hi);
    let pivot = hi;
    let mut i = lo;
    let mut j = hi - 1;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }
        while j > 0 && arr[j] > arr[pivot] {
            j -= 1;
        }
        if j == 0 || i >= j {
            break;
        } else if arr[i] == arr[j] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j);
        }
    }
    arr.swap(i, pivot);
	log::debug!("<<< Exiting partition(lo={},hi={})", lo, hi);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize) {
	log::debug!(">>> Entering _quick_sort(lo={},hi={})", lo, hi);
    while lo < hi {
        let pivot = partition(arr, lo, hi);
		log::debug!("_quick_sort(lo={},hi={}): pivot → {}", lo, hi, pivot);

        if pivot - lo < hi - pivot {
            if pivot > 0 {
                _quick_sort(arr, lo, pivot - 1);
            }
            lo = pivot + 1;
        } else {
            _quick_sort(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
	log::debug!("<<< Exiting _quick_sort(lo={},hi={})", lo, hi);
    }
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
	log::debug!(">>> Entering quick_sort()");
    let len = arr.len();
    if len > 1 {
		log::debug!("quick_sort(): len → {}", len);
        _quick_sort(arr, 0, len - 1);
	log::debug!("<<< Exiting quick_sort()");
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
		log::debug!(">>> Entering basic()");
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let cloned = res.clone();
        quick_sort(&mut res);
		log::debug!("<<< Exiting basic()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn basic_string() {
		log::debug!(">>> Entering basic_string()");
        let mut res = vec!["a", "bb", "d", "cc"];
        let cloned = res.clone();
        quick_sort(&mut res);
		log::debug!("<<< Exiting basic_string()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        quick_sort(&mut res);
		log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
		log::debug!(">>> Entering one_element()");
        let mut res = vec![1];
        let cloned = res.clone();
        quick_sort(&mut res);
		log::debug!("<<< Exiting one_element()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
		log::debug!(">>> Entering pre_sorted()");
        let mut res = vec![1, 2, 3, 4];
        let cloned = res.clone();
        quick_sort(&mut res);
		log::debug!("<<< Exiting pre_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn reverse_sorted() {
		log::debug!(">>> Entering reverse_sorted()");
        let mut res = vec![4, 3, 2, 1];
        let cloned = res.clone();
        quick_sort(&mut res);
		log::debug!("<<< Exiting reverse_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn large_elements() {
		log::debug!(">>> Entering large_elements()");
        let mut res = sort_utils::generate_random_vec(300000, 0, 1000000);
        let cloned = res.clone();
        sort_utils::log_timed("large elements test", || {
            quick_sort(&mut res);
        });
		log::debug!("<<< Exiting large_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn nearly_ordered_elements() {
		log::debug!(">>> Entering nearly_ordered_elements()");
        let mut res = sort_utils::generate_nearly_ordered_vec(3000, 10);
        let cloned = res.clone();

        sort_utils::log_timed("nearly ordered elements test", || {
            quick_sort(&mut res);
        });

		log::debug!("<<< Exiting nearly_ordered_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn repeated_elements() {
		log::debug!(">>> Entering repeated_elements()");
        let mut res = sort_utils::generate_repeated_elements_vec(1000000, 3);
        let cloned = res.clone();

        sort_utils::log_timed("repeated elements test", || {
            quick_sort(&mut res);
        });

		log::debug!("<<< Exiting repeated_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
