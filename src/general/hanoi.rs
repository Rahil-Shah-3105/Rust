pub fn hanoi(n: i32, from: i32, to: i32, via: i32, moves: &mut Vec<(i32, i32)>) {
	eprintln!("[ERROR] >>> Entering hanoi(n={},from={},to={},via={},moves={:?})", n, from, to, via, moves);
    if n > 0 {
        hanoi(n - 1, from, via, to, moves);
        moves.push((from, to));
		eprintln!("[WARN] <<< Exiting hanoi(n={},from={},to={},via={},moves={:?})", n, from, to, via, moves);
        hanoi(n - 1, via, to, from, moves);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hanoi_simple() {
		eprintln!("[ERROR] >>> Entering hanoi_simple()");
        let correct_solution: Vec<(i32, i32)> =
            vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)];
        let mut our_solution: Vec<(i32, i32)> = Vec::new();
        hanoi(3, 1, 3, 2, &mut our_solution);
		eprintln!("[WARN] <<< Exiting hanoi_simple()");
        assert_eq!(correct_solution, our_solution);
    }
}
