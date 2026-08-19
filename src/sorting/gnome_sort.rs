use std::cmp;

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    let mut arr = arr.to_vec();
    let mut i: usize = 1;
    let mut j: usize = 2;

    while i < arr.len() {
		log::debug!(">>> Entering gnome_sort(arr=(unprintable))");
		log::debug!("gnome_sort(arr=(unprintable)): arr → (unprintable)");
        if arr[i - 1] < arr[i] {
            i = j;
            j = i + 1;
        } else {
            arr.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
		log::debug!(">>> Entering basic()");
        let original = [6, 5, -8, 3, 2, 3];
        let res = gnome_sort(&original);
		log::debug!("basic(): res → (unprintable)");
		log::debug!("<<< Exiting basic()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn already_sorted() {
		log::debug!(">>> Entering already_sorted()");
        let original = gnome_sort(&["a", "b", "c"]);
        let res = gnome_sort(&original);
		log::debug!("already_sorted(): res → (unprintable)");
		log::debug!("already_sorted(): original → (unprintable)");
    log::debug!("<<< Exiting already_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn odd_number_of_elements() {
		log::debug!(">>> Entering odd_number_of_elements()");
        let original = gnome_sort(&["d", "a", "c", "e", "b"]);
        let res = gnome_sort(&original);
		log::debug!("odd_number_of_elements(): res → (unprintable)");
		log::debug!("odd_number_of_elements(): original → (unprintable)");
    log::debug!("<<< Exiting odd_number_of_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn one_element() {
		log::debug!(">>> Entering one_element()");
        let original = gnome_sort(&[3]);
        let res = gnome_sort(&original);
		log::debug!("one_element(): res → (unprintable)");
		log::debug!("one_element(): original → (unprintable)");
    log::debug!("<<< Exiting one_element()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let original = gnome_sort(&Vec::<u8>::new());
        let res = gnome_sort(&original);
		log::debug!("empty(): res → (unprintable)");
		log::debug!("empty(): original → (unprintable)");
    log::debug!("<<< Exiting empty()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }
}
