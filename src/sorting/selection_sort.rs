pub fn selection_sort<T: Ord>(arr: &mut [T]) {
	log::debug!(">>> Entering selection_sort(arr=(unprintable))");
    let len = arr.len();
    for left in 0..len {
		log::debug!("selection_sort(arr=(unprintable)): len → {}", len);
		log::debug!("selection_sort(arr=(unprintable)): smallest → (unprintable)");
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
	log::debug!("<<< Exiting selection_sort(arr=(unprintable))");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
		log::debug!(">>> Entering basic()");
        let mut res = vec!["d", "a", "c", "b"];
        let cloned = res.clone();
		log::debug!("basic(): cloned → (unprintable)");
		log::debug!("basic(): res → (unprintable)");
        selection_sort(&mut res);
		log::debug!("<<< Exiting basic()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
		log::debug!("empty(): cloned → (unprintable)");
		log::debug!("empty(): res → (unprintable)");
        selection_sort(&mut res);
		log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
		log::debug!(">>> Entering one_element()");
        let mut res = vec!["a"];
        let cloned = res.clone();
		log::debug!("one_element(): cloned → (unprintable)");
		log::debug!("one_element(): res → (unprintable)");
        selection_sort(&mut res);
		log::debug!("<<< Exiting one_element()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
		log::debug!(">>> Entering pre_sorted()");
        let mut res = vec!["a", "b", "c"];
        let cloned = res.clone();
		log::debug!("pre_sorted(): cloned → (unprintable)");
		log::debug!("pre_sorted(): res → (unprintable)");
        selection_sort(&mut res);
		log::debug!("<<< Exiting pre_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
