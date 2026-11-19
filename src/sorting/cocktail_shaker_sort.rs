pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering cocktail_shaker_sort()");
    let len = arr.len();

	eprintln!("[ERROR] cocktail_shaker_sort(): len → {}", len);
    if len == 0 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 0..(len - 1).clamp(0, len) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
				eprintln!("[ERROR] cocktail_shaker_sort(): swapped → {}", swapped);
            }
        }

        if !swapped {
            break;
        }

        swapped = false;

        for i in (0..(len - 1).clamp(0, len)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
			eprintln!("[WARN] <<< Exiting cocktail_shaker_sort()");
            break;
        }
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
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting basic()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting empty()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
		eprintln!("[ERROR] >>> Entering one_element()");
        let mut arr = vec![1];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting one_element()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
		eprintln!("[ERROR] >>> Entering pre_sorted()");
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
		eprintln!("[WARN] <<< Exiting pre_sorted()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
