pub fn comb_sort<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering comb_sort()");
    let mut gap = arr.len();
    let shrink = 1.3;
	eprintln!("[ERROR] comb_sort(): gap → {}", gap);
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
			eprintln!("[ERROR] comb_sort(): sorted → {}", sorted);
        }
        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
	eprintln!("[WARN] <<< Exiting comb_sort()");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
		eprintln!("[ERROR] >>> Entering descending()");
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        comb_sort(&mut ve1);
		eprintln!("[WARN] <<< Exiting descending()");
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
		eprintln!("[ERROR] >>> Entering ascending()");
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        comb_sort(&mut ve2);
		eprintln!("[WARN] <<< Exiting ascending()");
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn duplicates() {
		eprintln!("[ERROR] >>> Entering duplicates()");
        //pre-sorted
        let mut ve3 = vec![2, 2, 2, 2, 2, 1];
        let cloned = ve3.clone();
        comb_sort(&mut ve3);
		eprintln!("[WARN] <<< Exiting duplicates()");
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
