fn _stooge_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
	eprintln!("[ERROR] >>> Entering _stooge_sort(start={},end={})", start, end);
    eprintln!("[ERROR] >>> Entering _stooge_sort(start={},end={})", start, end);
    if arr[start] > arr[end] {
        arr.swap(start, end);
    }

    if start + 1 >= end {
        return;
    }

    let k = (end - start + 1) / 3;

    _stooge_sort(arr, start, end - k);
    _stooge_sort(arr, start + k, end);
	eprintln!("[WARN] <<< Exiting _stooge_sort(start={},end={})", start, end);
    _stooge_sort(arr, start, end - k);
}

pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering stooge_sort()");
    let len = arr.len();
    if len == 0 {
		eprintln!("[ERROR] stooge_sort(): len → {}", len);
        return;
    }

	eprintln!("[WARN] <<< Exiting stooge_sort()");
    _stooge_sort(arr, 0, len - 1);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
		eprintln!("[ERROR] >>> Entering basic()");
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		eprintln!("[WARN] <<< Exiting basic()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		eprintln!("[WARN] <<< Exiting empty()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
		eprintln!("[ERROR] >>> Entering reverse()");
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		eprintln!("[WARN] <<< Exiting reverse()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
		eprintln!("[ERROR] >>> Entering already_sorted()");
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		eprintln!("[WARN] <<< Exiting already_sorted()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
