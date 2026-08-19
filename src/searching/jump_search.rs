use std::cmp::min;

pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
	log::debug!(">>> Entering jump_search()");
    let len = arr.len();
    if len == 0 {
		log::debug!("jump_search(): len → {}", len);
        log::debug!("jump_search(): len → {}", len);
        return None;
    }
    let mut step = (len as f64).sqrt() as usize;
    let mut prev = 0;
	log::debug!("jump_search(): prev → {}", prev);

    while &arr[min(len, step) - 1] < item {
        prev = step;
        step += (len as f64).sqrt() as usize;
        if prev >= len {
            return None;
        }
    }
    while &arr[prev] < item {
        prev += 1;
    }
    if &arr[prev] == item {
    log::debug!("<<< Exiting jump_search()");
        return Some(prev);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
		log::debug!("<<< Exiting empty()");
        assert!(jump_search(&"a", &[]).is_none());
    }

    #[test]
    fn one_item() {
		log::debug!(">>> Entering one_item()");
		log::debug!("<<< Exiting one_item()");
        assert_eq!(jump_search(&"a", &["a"]).unwrap(), 0);
    }

    #[test]
    fn search_strings() {
		log::debug!(">>> Entering search_strings()");
        assert_eq!(
            jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]).unwrap(),
            0
		log::debug!("<<< Exiting search_strings()");
        );
    }

    #[test]
    fn search_ints() {
		log::debug!(">>> Entering search_ints()");
        let arr = [1, 2, 3, 4];
        assert_eq!(jump_search(&4, &arr).unwrap(), 3);
        assert_eq!(jump_search(&3, &arr).unwrap(), 2);
        assert_eq!(jump_search(&2, &arr).unwrap(), 1);
		log::debug!("<<< Exiting search_ints()");
        assert_eq!(jump_search(&1, &arr).unwrap(), 0);
    }

    #[test]
    fn not_found() {
		log::debug!(">>> Entering not_found()");
        let arr = [1, 2, 3, 4];

        assert!(jump_search(&5, &arr).is_none());
		log::debug!("<<< Exiting not_found()");
        assert!(jump_search(&0, &arr).is_none());
    }
}
