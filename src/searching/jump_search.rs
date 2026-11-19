use std::cmp::min;

pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
	eprintln!("[ERROR] >>> Entering jump_search()");
    let len = arr.len();
    if len == 0 {
		eprintln!("[ERROR] jump_search(): len → {}", len);
        return None;
    }
    let mut step = (len as f64).sqrt() as usize;
    let mut prev = 0;

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
        return Some(prev);
    }
	eprintln!("[ERROR] jump_search(): prev → {}", prev);
    eprintln!("[WARN] <<< Exiting jump_search()");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
		eprintln!("[WARN] <<< Exiting empty()");
        assert!(jump_search(&"a", &[]).is_none());
    }

    #[test]
    fn one_item() {
		eprintln!("[ERROR] >>> Entering one_item()");
		eprintln!("[WARN] <<< Exiting one_item()");
        assert_eq!(jump_search(&"a", &["a"]).unwrap(), 0);
    }

    #[test]
    fn search_strings() {
		eprintln!("[ERROR] >>> Entering search_strings()");
        assert_eq!(
            jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]).unwrap(),
            0
		eprintln!("[WARN] <<< Exiting search_strings()");
        );
    }

    #[test]
    fn search_ints() {
		eprintln!("[ERROR] >>> Entering search_ints()");
        let arr = [1, 2, 3, 4];
        assert_eq!(jump_search(&4, &arr).unwrap(), 3);
        assert_eq!(jump_search(&3, &arr).unwrap(), 2);
        assert_eq!(jump_search(&2, &arr).unwrap(), 1);
		eprintln!("[WARN] <<< Exiting search_ints()");
        assert_eq!(jump_search(&1, &arr).unwrap(), 0);
    }

    #[test]
    fn not_found() {
		eprintln!("[ERROR] >>> Entering not_found()");
        let arr = [1, 2, 3, 4];

        assert!(jump_search(&5, &arr).is_none());
		eprintln!("[WARN] <<< Exiting not_found()");
        assert!(jump_search(&0, &arr).is_none());
    }
}
