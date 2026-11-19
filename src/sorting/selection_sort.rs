pub fn selection_sort<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering selection_sort()");
    let len = arr.len();
    for left in 0..len {
		eprintln!("[ERROR] selection_sort(): len → {}", len);
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
	eprintln!("[WARN] <<< Exiting selection_sort()");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
		eprintln!("[ERROR] >>> Entering basic()");
        let mut res = vec!["d", "a", "c", "b"];
        let cloned = res.clone();
        selection_sort(&mut res);
		eprintln!("[WARN] <<< Exiting basic()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let mut res = Vec::<u8>::new();
        let cloned = res.clone();
        selection_sort(&mut res);
		eprintln!("[WARN] <<< Exiting empty()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn one_element() {
		eprintln!("[ERROR] >>> Entering one_element()");
        let mut res = vec!["a"];
        let cloned = res.clone();
        selection_sort(&mut res);
		eprintln!("[WARN] <<< Exiting one_element()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }

    #[test]
    fn pre_sorted() {
		eprintln!("[ERROR] >>> Entering pre_sorted()");
        let mut res = vec!["a", "b", "c"];
        let cloned = res.clone();
        selection_sort(&mut res);
		eprintln!("[WARN] <<< Exiting pre_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
    }
}
