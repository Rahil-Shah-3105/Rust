/// Sorts a mutable slice using in-place insertion sort algorithm.
///
/// Time complexity is `O(n^2)`, where `n` is the number of elements.
/// Space complexity is `O(1)` as it sorts elements in-place.
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
	log::debug!(">>> Entering insertion_sort(arr=(unprintable))");
    for i in 1..arr.len() {
        let mut j = i;
        let cur = arr[i];

        while j > 0 && cur < arr[j - 1] {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = cur;
	log::debug!("<<< Exiting insertion_sort(arr=(unprintable))");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let mut arr: [u8; 0] = [];
        let cloned = arr;
        insertion_sort(&mut arr);
		log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
		log::debug!(">>> Entering one_element()");
        let mut arr: [char; 1] = ['a'];
        let cloned = arr;
        insertion_sort(&mut arr);
		log::debug!("<<< Exiting one_element()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn already_sorted() {
		log::debug!(">>> Entering already_sorted()");
        let mut arr: [&str; 3] = ["a", "b", "c"];
        let cloned = arr;
        insertion_sort(&mut arr);
		log::debug!("<<< Exiting already_sorted()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn basic() {
		log::debug!(">>> Entering basic()");
        let mut arr: [&str; 4] = ["d", "a", "c", "b"];
        let cloned = arr;
        insertion_sort(&mut arr);
		log::debug!("<<< Exiting basic()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn odd_number_of_elements() {
		log::debug!(">>> Entering odd_number_of_elements()");
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        let cloned = arr.clone();
		log::debug!("odd_number_of_elements(): cloned → (unprintable)");
        insertion_sort(&mut arr);
		log::debug!("<<< Exiting odd_number_of_elements()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn repeated_elements() {
		log::debug!(">>> Entering repeated_elements()");
        let mut arr: Vec<usize> = vec![542, 542, 542, 542];
        let cloned = arr.clone();
		log::debug!("repeated_elements(): cloned → (unprintable)");
        insertion_sort(&mut arr);
		log::debug!("<<< Exiting repeated_elements()");
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
