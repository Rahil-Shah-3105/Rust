use std::cmp;

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
	eprintln!("[ERROR] >>> Entering gnome_sort()");
    let mut arr = arr.to_vec();
    let mut i: usize = 1;
    let mut j: usize = 2;

    while i < arr.len() {
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
	eprintln!("[WARN] <<< Exiting gnome_sort()");
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
		eprintln!("[ERROR] >>> Entering basic()");
        let original = [6, 5, -8, 3, 2, 3];
        let res = gnome_sort(&original);
		eprintln!("[WARN] <<< Exiting basic()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn already_sorted() {
		eprintln!("[ERROR] >>> Entering already_sorted()");
        let original = gnome_sort(&["a", "b", "c"]);
        let res = gnome_sort(&original);
		eprintln!("[WARN] <<< Exiting already_sorted()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn odd_number_of_elements() {
		eprintln!("[ERROR] >>> Entering odd_number_of_elements()");
        let original = gnome_sort(&["d", "a", "c", "e", "b"]);
        let res = gnome_sort(&original);
		eprintln!("[WARN] <<< Exiting odd_number_of_elements()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn one_element() {
		eprintln!("[ERROR] >>> Entering one_element()");
        let original = gnome_sort(&[3]);
        let res = gnome_sort(&original);
		eprintln!("[WARN] <<< Exiting one_element()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let original = gnome_sort(&Vec::<u8>::new());
        let res = gnome_sort(&original);
		eprintln!("[WARN] <<< Exiting empty()");
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }
}
