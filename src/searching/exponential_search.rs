use std::cmp::Ordering;

pub fn exponential_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
	log::debug!(">>> Entering exponential_search()");
    let len = arr.len();
    if len == 0 {
        return None;
    }
    let mut upper = 1;
    while (upper < len) && (&arr[upper] <= item) {
        upper *= 2;
    }
    if upper > len {
        upper = len
    }

    // binary search
    let mut lower = upper / 2;
    while lower < upper {
        let mid = lower + (upper - lower) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Less => upper = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => lower = mid + 1,
        }
    }
	log::debug!("exponential_search(): len → {}", len);
    log::debug!("exponential_search(): len → {}", len);
    log::debug!("<<< Exiting exponential_search()");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let index = exponential_search(&"a", &[]);
		log::debug!("<<< Exiting empty()");
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
		log::debug!(">>> Entering one_item()");
        let index = exponential_search(&"a", &["a"]);
		log::debug!("<<< Exiting one_item()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
		log::debug!(">>> Entering search_strings()");
        let index = exponential_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
		log::debug!("<<< Exiting search_strings()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
		log::debug!(">>> Entering search_ints()");
        let index = exponential_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = exponential_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = exponential_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = exponential_search(&1, &[1, 2, 3, 4]);
		log::debug!("<<< Exiting search_ints()");
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
		log::debug!(">>> Entering not_found()");
        let index = exponential_search(&5, &[1, 2, 3, 4]);
		log::debug!("<<< Exiting not_found()");
        assert_eq!(index, None);
    }
}
