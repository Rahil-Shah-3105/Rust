fn _comp_and_swap<T: Ord>(array: &mut [T], left: usize, right: usize, ascending: bool) {
	log::debug!(">>> Entering _comp_and_swap(left={},right={},ascending={})", left, right, ascending);
    if (ascending && array[left] > array[right]) || (!ascending && array[left] < array[right]) {
        array.swap(left, right);
	log::debug!("<<< Exiting _comp_and_swap(left={},right={},ascending={})", left, right, ascending);
    }
}

fn _bitonic_merge<T: Ord>(array: &mut [T], low: usize, length: usize, ascending: bool) {
	log::debug!(">>> Entering _bitonic_merge(low={},length={},ascending={})", low, length, ascending);
    if length > 1 {
        let middle = length / 2;
        for i in low..(low + middle) {
            _comp_and_swap(array, i, i + middle, ascending);
        }
        _bitonic_merge(array, low, middle, ascending);
        _bitonic_merge(array, low + middle, middle, ascending);
	log::debug!("<<< Exiting _bitonic_merge(low={},length={},ascending={})", low, length, ascending);
    }
}

pub fn bitonic_sort<T: Ord>(array: &mut [T], low: usize, length: usize, ascending: bool) {
	log::debug!(">>> Entering bitonic_sort(low={},length={},ascending={})", low, length, ascending);
    if length > 1 {
        let middle = length / 2;
        bitonic_sort(array, low, middle, true);
        bitonic_sort(array, low + middle, middle, false);
        _bitonic_merge(array, low, length, ascending);
	log::debug!("<<< Exiting bitonic_sort(low={},length={},ascending={})", low, length, ascending);
    }
}

//Note that this program works only when size of input is a power of 2.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_descending_sorted;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
		log::debug!(">>> Entering descending()");
        //descending
        let mut ve1 = vec![6, 5, 4, 3];
        let cloned = ve1.clone();
        bitonic_sort(&mut ve1, 0, 4, true);
		log::debug!("<<< Exiting descending()");
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
		log::debug!(">>> Entering ascending()");
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4];
        let cloned = ve2.clone();
        bitonic_sort(&mut ve2, 0, 4, false);
		log::debug!("<<< Exiting ascending()");
        assert!(is_descending_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
