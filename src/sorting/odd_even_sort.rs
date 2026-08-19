pub fn odd_even_sort<T: Ord>(arr: &mut [T]) {
	log::debug!(">>> Entering odd_even_sort(arr=(unprintable))");
    let len = arr.len();
    if len == 0 {
		log::debug!("odd_even_sort(arr=(unprintable)): len → {}", len);
        log::debug!("odd_even_sort(arr=(unprintable)): len → {}", len);
        return;
    }

    let mut sorted = false;
	log::debug!("odd_even_sort(arr=(unprintable)): sorted → {}", sorted);
    while !sorted {
        sorted = true;

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
	log::debug!("<<< Exiting odd_even_sort(arr=(unprintable))");
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
        let mut arr = vec![3, 5, 1, 2, 4, 6];
        let cloned = arr.clone();
		log::debug!("basic(): cloned → (unprintable)");
        odd_even_sort(&mut arr);
		log::debug!("<<< Exiting basic()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
		log::debug!("empty(): cloned → (unprintable)");
		log::debug!("empty(): arr → (unprintable)");
        odd_even_sort(&mut arr);
		log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
		log::debug!(">>> Entering one_element()");
        let mut arr = vec![3];
        let cloned = arr.clone();
		log::debug!("one_element(): cloned → (unprintable)");
        odd_even_sort(&mut arr);
		log::debug!("<<< Exiting one_element()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
		log::debug!(">>> Entering pre_sorted()");
        let mut arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = arr.clone();
		log::debug!("pre_sorted(): cloned → (unprintable)");
        odd_even_sort(&mut arr);
		log::debug!("<<< Exiting pre_sorted()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
