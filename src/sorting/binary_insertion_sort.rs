fn _binary_search<T: Ord>(arr: &[T], target: &T) -> usize {
	eprintln!("[ERROR] >>> Entering _binary_search()");
    let mut low = 0;
    let mut high = arr.len();
	eprintln!("[ERROR] _binary_search(): high → {}", high);

    while low < high {
        let mid = low + (high - low) / 2;

        if arr[mid] < *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

	eprintln!("[WARN] <<< Exiting _binary_search()");
    low
}

pub fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering binary_insertion_sort()");
    let len = arr.len();

	eprintln!("[ERROR] binary_insertion_sort(): len → {}", len);
    for i in 1..len {
        let key = arr[i].clone();
        let index = _binary_search(&arr[..i], &key);

        arr[index..=i].rotate_right(1);
	eprintln!("[WARN] <<< Exiting binary_insertion_sort()");
        arr[index] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_insertion_sort() {
		eprintln!("[ERROR] >>> Entering test_binary_insertion_sort()");
        let mut arr1 = vec![64, 25, 12, 22, 11];
        let mut arr2 = vec![5, 4, 3, 2, 1];
        let mut arr3 = vec![1, 2, 3, 4, 5];
        let mut arr4: Vec<i32> = vec![]; // Explicitly specify the type for arr4

        binary_insertion_sort(&mut arr1);
        binary_insertion_sort(&mut arr2);
        binary_insertion_sort(&mut arr3);
        binary_insertion_sort(&mut arr4);

        assert_eq!(arr1, vec![11, 12, 22, 25, 64]);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr3, vec![1, 2, 3, 4, 5]);
		eprintln!("[WARN] <<< Exiting test_binary_insertion_sort()");
        assert_eq!(arr4, Vec::<i32>::new());
    }
}
