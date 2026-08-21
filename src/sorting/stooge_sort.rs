fn _stooge_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    if arr[start] > arr[end] {
        arr.swap(start, end);
    }

    if start + 1 >= end {
        return;
    }

    let k = (end - start + 1) / 3;

    _stooge_sort(arr, start, end - k);
    _stooge_sort(arr, start + k, end);
    _stooge_sort(arr, start, end - k);
}

pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }

    _stooge_sort(arr, 0, len - 1);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
		log::debug!(">>> Entering basic()");
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		log::debug!("<<< Exiting basic()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
		log::debug!(">>> Entering reverse()");
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		log::debug!("<<< Exiting reverse()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
		log::debug!(">>> Entering already_sorted()");
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
		log::debug!("<<< Exiting already_sorted()");
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
