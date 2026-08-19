use std::cmp::min;
use std::cmp::Ordering;

pub fn fibonacci_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
	log::debug!(">>> Entering fibonacci_search(item=(unprintable),arr=(unprintable))");
    let len = arr.len();
    if len == 0 {
		log::debug!("fibonacci_search(item=(unprintable),arr=(unprintable)): len → {}", len);
        log::debug!("fibonacci_search(item=(unprintable),arr=(unprintable)): len → {}", len);
        return None;
    }
    let mut start = -1;

    let mut f0 = 0;
	log::debug!("fibonacci_search(item=(unprintable),arr=(unprintable)): f0 → {}", f0);
    let mut f1 = 1;
	log::debug!("fibonacci_search(item=(unprintable),arr=(unprintable)): f1 → {}", f1);
    let mut f2 = 1;
	log::debug!("fibonacci_search(item=(unprintable),arr=(unprintable)): f2 → {}", f2);
    while f2 < len {
        f0 = f1;
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
    log::debug!("<<< Exiting fibonacci_search(item=(unprintable),arr=(unprintable))");
        return Some(len - 1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let index = fibonacci_search(&"a", &[]);
		log::debug!("<<< Exiting empty()");
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
		log::debug!(">>> Entering one_item()");
        let index = fibonacci_search(&"a", &["a"]);
		log::debug!("<<< Exiting one_item()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
		log::debug!(">>> Entering search_strings()");
        let index = fibonacci_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
		log::debug!("<<< Exiting search_strings()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
		log::debug!(">>> Entering search_ints()");
        let index = fibonacci_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = fibonacci_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = fibonacci_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = fibonacci_search(&1, &[1, 2, 3, 4]);
		log::debug!("<<< Exiting search_ints()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
		log::debug!(">>> Entering not_found()");
        let index = fibonacci_search(&5, &[1, 2, 3, 4]);
		log::debug!("<<< Exiting not_found()");
        assert_eq!(index, None);
    }
}
