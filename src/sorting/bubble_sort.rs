pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
	log::debug!(">>> Entering bubble_sort(arr=(unprintable))");
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
	log::debug!("bubble_sort(arr=(unprintable)): sorted → {}", sorted);
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
	log::debug!("<<< Exiting bubble_sort(arr=(unprintable))");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
		log::debug!(">>> Entering descending()");
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
		log::debug!("descending(): cloned → (unprintable)");
        bubble_sort(&mut ve1);
		log::debug!("<<< Exiting descending()");
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
		log::debug!(">>> Entering ascending()");
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
		log::debug!("ascending(): cloned → (unprintable)");
        bubble_sort(&mut ve2);
		log::debug!("<<< Exiting ascending()");
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
		log::debug!("empty(): cloned → (unprintable)");
        bubble_sort(&mut ve3);
		log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
