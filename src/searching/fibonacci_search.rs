use std::cmp::min;
use std::cmp::Ordering;

pub fn fibonacci_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
	eprintln!("[ERROR] >>> Entering fibonacci_search()");
    let len = arr.len();
    if len == 0 {
		eprintln!("[ERROR] fibonacci_search(): len → {}", len);
        return None;
    }
    let mut start = -1;

    let mut f0 = 0;
    let mut f1 = 1;
	eprintln!("[ERROR] fibonacci_search(): f1 → {}", f1);
    let mut f2 = 1;
	eprintln!("[ERROR] fibonacci_search(): f2 → {}", f2);
    while f2 < len {
        f0 = f1;
		eprintln!("[ERROR] fibonacci_search(): f0 → {}", f0);
        f1 = f2;
        f2 = f0 + f1;
    }
    while f2 > 1 {
        let index = min((f0 as isize + start) as usize, len - 1);
        match item.cmp(&arr[index]) {
            Ordering::Less => {
                f2 = f0;
                f1 -= f0;
                f0 = f2 - f1;
            }
            Ordering::Equal => return Some(index),
            Ordering::Greater => {
                f2 = f1;
                f1 = f0;
                f0 = f2 - f1;
                start = index as isize;
            }
        }
    }
    if (f1 != 0) && (&arr[len - 1] == item) {
        return Some(len - 1);
    }
    eprintln!("[WARN] <<< Exiting fibonacci_search()");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let index = fibonacci_search(&"a", &[]);
		eprintln!("[WARN] <<< Exiting empty()");
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
		eprintln!("[ERROR] >>> Entering one_item()");
        let index = fibonacci_search(&"a", &["a"]);
		eprintln!("[WARN] <<< Exiting one_item()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
		eprintln!("[ERROR] >>> Entering search_strings()");
        let index = fibonacci_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
		eprintln!("[WARN] <<< Exiting search_strings()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
		eprintln!("[ERROR] >>> Entering search_ints()");
        let index = fibonacci_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = fibonacci_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = fibonacci_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = fibonacci_search(&1, &[1, 2, 3, 4]);
		eprintln!("[WARN] <<< Exiting search_ints()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
		eprintln!("[ERROR] >>> Entering not_found()");
        let index = fibonacci_search(&5, &[1, 2, 3, 4]);
		eprintln!("[WARN] <<< Exiting not_found()");
        assert_eq!(index, None);
    }
}
