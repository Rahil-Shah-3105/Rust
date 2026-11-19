use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn sleep_sort(vec: &[usize]) -> Vec<usize> {
	eprintln!("[ERROR] >>> Entering sleep_sort(vec={:?})", vec);
    let len = vec.len();
    let (tx, rx) = mpsc::channel();
	eprintln!("[ERROR] sleep_sort(vec={:?}): len → {}", vec, len);

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

	eprintln!("[WARN] <<< Exiting sleep_sort(vec={:?})", vec);
    sorted_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
		eprintln!("[ERROR] >>> Entering empty()");
        let res = sleep_sort(&[]);
		eprintln!("[ERROR] empty(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting empty()");
        assert_eq!(res, &[]);
    }

    #[test]
    fn single_element() {
		eprintln!("[ERROR] >>> Entering single_element()");
        let res = sleep_sort(&[1]);
		eprintln!("[ERROR] single_element(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting single_element()");
        assert_eq!(res, &[1]);
    }

    #[test]
    fn sorted_array() {
		eprintln!("[ERROR] >>> Entering sorted_array()");
        let res = sleep_sort(&[1, 2, 3, 4]);
		eprintln!("[ERROR] sorted_array(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting sorted_array()");
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn unsorted_array() {
		eprintln!("[ERROR] >>> Entering unsorted_array()");
        let res = sleep_sort(&[3, 4, 2, 1]);
		eprintln!("[ERROR] unsorted_array(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting unsorted_array()");
        assert_eq!(res, &[1, 2, 3, 4]);
    }

    #[test]
    fn odd_number_of_elements() {
		eprintln!("[ERROR] >>> Entering odd_number_of_elements()");
        let res = sleep_sort(&[3, 1, 7]);
		eprintln!("[ERROR] odd_number_of_elements(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting odd_number_of_elements()");
        assert_eq!(res, &[1, 3, 7]);
    }

    #[test]
    fn repeated_elements() {
		eprintln!("[ERROR] >>> Entering repeated_elements()");
        let res = sleep_sort(&[1, 1, 1, 1]);
		eprintln!("[ERROR] repeated_elements(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting repeated_elements()");
        assert_eq!(res, &[1, 1, 1, 1]);
    }

    #[test]
    fn random_elements() {
		eprintln!("[ERROR] >>> Entering random_elements()");
        let res = sleep_sort(&[5, 3, 7, 10, 1, 0, 8]);
		eprintln!("[ERROR] random_elements(): res → {:?}", res);
    eprintln!("[WARN] <<< Exiting random_elements()");
        assert_eq!(res, &[0, 1, 3, 5, 7, 8, 10]);
    }
}
