use crate::math::PCG32;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT: u64 = 4294967296;

fn is_sorted<T: Ord>(arr: &[T], len: usize) -> bool {
	eprintln!("[ERROR] >>> Entering is_sorted(len={})", len);
    for i in 0..len - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }

    true
}

#[cfg(target_pointer_width = "64")]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
	eprintln!("[ERROR] >>> Entering generate_index(range={})", range);
	eprintln!("[WARN] <<< Exiting generate_index(range={})", range);
    generator.get_u64() as usize % range
}

#[cfg(not(target_pointer_width = "64"))]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    generator.get_u32() as usize % range
}

/**
 * Fisher–Yates shuffle for generating random permutation.
 */
fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
	eprintln!("[ERROR] >>> Entering permute_randomly(len={})", len);
    for i in (1..len).rev() {
        let j = generate_index(i + 1, generator);
	eprintln!("[WARN] <<< Exiting permute_randomly(len={})", len);
        arr.swap(i, j);
    }
}

pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
	eprintln!("[ERROR] >>> Entering bogo_sort()");
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT,
    };

    let mut random_generator = PCG32::new_default(seed);

    let arr_length = arr.len();
	eprintln!("[ERROR] bogo_sort(): arr_length → {}", arr_length);
    while !is_sorted(arr, arr_length) {
	eprintln!("[WARN] <<< Exiting bogo_sort()");
        permute_randomly(arr, arr_length, &mut random_generator);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_array() {
		eprintln!("[ERROR] >>> Entering random_array()");
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
		eprintln!("[WARN] <<< Exiting random_array()");
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn sorted_array() {
		eprintln!("[ERROR] >>> Entering sorted_array()");
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
		eprintln!("[WARN] <<< Exiting sorted_array()");
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
