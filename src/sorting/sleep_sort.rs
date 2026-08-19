use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sleep_sort(vec: &[usize]) -> Vec<usize> {
	log::debug!(">>> Entering sleep_sort(vec={:?})", vec);
    let len = vec.len();
    let (tx, rx) = mpsc::channel();
	log::debug!("sleep_sort(vec={:?}): len → {}", vec, len);

    for &x in vec.iter() {
        let tx: mpsc::Sender<usize> = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis((20 * x) as u64));
            tx.send(x).expect("panic");
        });
    }
    let mut sorted_list: Vec<usize> = Vec::new();

    for _ in 0..len {
        sorted_list.push(rx.recv().unwrap())
    }

	log::debug!("<<< Exiting sleep_sort(vec={:?})", vec);
    sorted_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		log::debug!(">>> Entering empty()");
        let res = sleep_sort(&[]);
		log::debug!("empty(): res → {:?}", res);
    log::debug!("<<< Exiting empty()");
        assert_eq!(res, &[]);
    }

    #[test]
    fn single_element() {
		log::debug!(">>> Entering single_element()");
        let res = sleep_sort(&[1]);
		log::debug!("single_element(): res → {:?}", res);
    log::debug!("<<< Exiting single_element()");
        assert_eq!(res, &[1]);
    }

    #[test]
    fn sorted_array() {
		log::debug!(">>> Entering sorted_array()");
        let res = sleep_sort(&[1, 2, 3, 4]);
		log::debug!("sorted_array(): res → {:?}", res);
    log::debug!("<<< Exiting sorted_array()");
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
		log::debug!(">>> Entering unsorted_array()");
        let res = sleep_sort(&[3, 4, 2, 1]);
		log::debug!("unsorted_array(): res → {:?}", res);
    log::debug!("<<< Exiting unsorted_array()");
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
		log::debug!(">>> Entering odd_number_of_elements()");
        let res = sleep_sort(&[3, 1, 7]);
		log::debug!("odd_number_of_elements(): res → {:?}", res);
    log::debug!("<<< Exiting odd_number_of_elements()");
        assert_eq!(res, &[1, 3, 7]);
    }

    #[test]
    fn repeated_elements() {
		log::debug!(">>> Entering repeated_elements()");
        let res = sleep_sort(&[1, 1, 1, 1]);
		log::debug!("repeated_elements(): res → {:?}", res);
    log::debug!("<<< Exiting repeated_elements()");
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
		log::debug!(">>> Entering random_elements()");
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
		log::debug!("random_elements(): res → {:?}", res);
    log::debug!("<<< Exiting random_elements()");
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }
}
