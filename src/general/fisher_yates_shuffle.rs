use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::PCG32;

const DEFAULT: u64 = 4294967296;

fn gen_range(range: usize, generator: &mut PCG32) -> usize {
	log::debug!(">>> Entering gen_range(range={},generator=(unprintable))", range);
    log::debug!("<<< Exiting gen_range(range={},generator=(unprintable))", range);
    generator.get_u64() as usize % range
}

pub fn fisher_yates_shuffle(array: &mut [i32]) {
	log::debug!(">>> Entering fisher_yates_shuffle(array={:?})", array);
    log::error!("Exception in fisher_yates_shuffle(array={:?}): (unprintable)", array);
    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() as u64,
        Err(_) => DEFAULT,
    };

    let mut random_generator = PCG32::new_default(seed);

    let len = array.len();
	log::debug!("fisher_yates_shuffle(array={:?}): len → {}", array, len);

    for i in 0..(len - 2) {
        let r = gen_range(len - i, &mut random_generator);
		log::debug!("fisher_yates_shuffle(array={:?}): r → {}", array, r);
        array.swap(i, i + r);
	log::debug!("<<< Exiting fisher_yates_shuffle(array={:?})", array);
    }
}
