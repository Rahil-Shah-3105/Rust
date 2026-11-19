pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering odd_even_sort()");
    let len = arr.len();
    if len == 0 {
		eprintln!("[ERROR] odd_even_sort(): len → {}", len);
        return;
    }

    let mut sorted = false;
    while !sorted {
        sorted = true;
		eprintln!("[ERROR] odd_even_sort(): sorted → {}", sorted);

        for i in (1..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }

        for i in (0..len - 1).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
	eprintln!("[WARN] <<< Exiting odd_even_sort()");
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
        let mut arr = vec![3, 5, 1, 2, 4, 6];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting basic()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting empty()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
		eprintln!("[ERROR] >>> Entering one_element()");
        let mut arr = vec![3];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting one_element()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
		eprintln!("[ERROR] >>> Entering pre_sorted()");
        let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = arr.clone();
        odd_even_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting pre_sorted()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
