fn _binary_search<T: Ord>(arr: &[T], target: &T) -> usize {
	log::debug!(">>> Entering _binary_search(arr=(unprintable),target=(unprintable))");
    let mut low = 0;
    let mut high = arr.len();
	log::debug!("_binary_search(arr=(unprintable),target=(unprintable)): high → {}", high);
	log::debug!("_binary_search(arr=(unprintable),target=(unprintable)): low → {}", low);

    while low < high {
        let mid = low + (high - low) / 2;
		log::debug!("_binary_search(arr=(unprintable),target=(unprintable)): mid → (unprintable)");

        if arr[mid] < *target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

	log::debug!("<<< Exiting _binary_search(arr=(unprintable),target=(unprintable))");
    low
}

pub fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
	log::debug!(">>> Entering binary_insertion_sort(arr=(unprintable))");
    let len = arr.len();

	log::debug!("binary_insertion_sort(arr=(unprintable)): len → {}", len);
    for i in 1..len {
        let key = arr[i].clone();
		log::warn!("binary_insertion_sort(arr=(unprintable)): key → (unprintable)");
        let index = _binary_search(&arr[..i], &key);

        arr[index..=i].rotate_right(1);
        arr[index] = key;
	log::debug!("<<< Exiting binary_insertion_sort(arr=(unprintable))");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_insertion_sort() {
		log::debug!(">>> Entering test_binary_insertion_sort()");
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
		log::debug!("<<< Exiting test_binary_insertion_sort()");
        assert_eq!(arr4, Vec::<i32>::new());
    }
}
