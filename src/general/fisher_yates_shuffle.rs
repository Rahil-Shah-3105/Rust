use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::PCG32;

const DEFAULT: u64 = 4294967296;

fn gen_range(range: usize, generator: &mut PCG32) -> usize {
	eprintln!("[ERROR] >>> Entering gen_range(range={})", range);
    generator.get_u64() as usize % range
}

pub fn fisher_yates_shuffle(array: &mut [i32]) {
	eprintln!("[ERROR] >>> Entering fisher_yates_shuffle(array={:?})", array);
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT,
    };

    let mut random_generator = PCG32::new_default(seed);

    let len = array.len();
	eprintln!("[ERROR] fisher_yates_shuffle(array={:?}): len → {}", array, len);

    for i in 0..(len - 2) {
        let r = gen_range(len - i, &mut random_generator);
		eprintln!("[ERROR] fisher_yates_shuffle(array={:?}): r → {}", array, r);
        array.swap(i, i + r);
	eprintln!("[WARN] <<< Exiting fisher_yates_shuffle(array={:?})", array);
    }
}
