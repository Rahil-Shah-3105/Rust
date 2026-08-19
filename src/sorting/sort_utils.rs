#[cfg(test)]
use rand::RngExt;
#[cfg(test)]
use std::time::Instant;

#[cfg(test)]
pub fn generate_random_vec(n: u32, range_l: i32, range_r: i32) -> Vec<i32> {
	log::debug!(">>> Entering generate_random_vec(n={},range_l={},range_r={})", n, range_l, range_r);
    let mut arr = Vec::<i32>::with_capacity(n as usize);
    let mut rng = rand::rng();
    let mut count = n;

    while count > 0 {
        arr.push(rng.random_range(range_l..=range_r));
        count -= 1;
    }

	log::debug!("<<< Exiting generate_random_vec(n={},range_l={},range_r={})", n, range_l, range_r);
    arr
}

#[cfg(test)]
pub fn generate_nearly_ordered_vec(n: u32, swap_times: u32) -> Vec<i32> {
	log::debug!(">>> Entering generate_nearly_ordered_vec(n={},swap_times={})", n, swap_times);
    let mut arr: Vec<i32> = (0..n as i32).collect();
    let mut rng = rand::rng();

    let mut count = swap_times;

    while count > 0 {
        arr.swap(
            rng.random_range(0..n as usize),
            rng.random_range(0..n as usize),
        );
        count -= 1;
    }

	log::debug!("<<< Exiting generate_nearly_ordered_vec(n={},swap_times={})", n, swap_times);
    arr
}

#[cfg(test)]
pub fn generate_ordered_vec(n: u32) -> Vec<i32> {
	log::debug!(">>> Entering generate_ordered_vec(n={})", n);
    log::debug!("<<< Exiting generate_ordered_vec(n={})", n);
    generate_nearly_ordered_vec(n, 0)
}

#[cfg(test)]
pub fn generate_reverse_ordered_vec(n: u32) -> Vec<i32> {
	log::debug!(">>> Entering generate_reverse_ordered_vec(n={})", n);
    let mut arr = generate_ordered_vec(n);
    arr.reverse();
	log::debug!("generate_reverse_ordered_vec(n={}): arr → {:?}", n, arr);
    log::debug!("<<< Exiting generate_reverse_ordered_vec(n={})", n);
    arr
}

#[cfg(test)]
pub fn generate_repeated_elements_vec(n: u32, unique_elements: u8) -> Vec<i32> {
	log::debug!(">>> Entering generate_repeated_elements_vec(n={},unique_elements={})", n, unique_elements);
    let mut rng = rand::rng();
    let v = rng.random_range(0..n as i32);
	log::debug!("generate_repeated_elements_vec(n={},unique_elements={}): v → (unprintable)", n, unique_elements);
    log::debug!("generate_repeated_elements_vec(n={},unique_elements={}): rng → (unprintable)", n, unique_elements);
    log::debug!("<<< Exiting generate_repeated_elements_vec(n={},unique_elements={})", n, unique_elements);
    generate_random_vec(n, v, v + unique_elements as i32)
}

#[cfg(test)]
pub fn log_timed<F>(test_name: &str, f: F)
where
	log::debug!(">>> Entering log_timed(test_name={},f=(unprintable))", test_name);
	log::debug!("<<< Exiting log_timed(test_name={},f=(unprintable))", test_name);
    F: FnOnce(),
{
    let before = Instant::now();
    f();
    println!("Elapsed time of {:?} is {:?}", test_name, before.elapsed());
}
