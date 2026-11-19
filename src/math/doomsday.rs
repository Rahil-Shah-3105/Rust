const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

pub fn doomsday(y: i32, m: i32, d: i32) -> i32 {
	eprintln!("[ERROR] >>> Entering doomsday(y={},m={},d={})", y, m, d);
    let y = if m < 3 { y - 1 } else { y };
	eprintln!("[WARN] <<< Exiting doomsday(y={},m={},d={})", y, m, d);
    (y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d) % 7
}

pub fn get_week_day(y: i32, m: i32, d: i32) -> String {
	eprintln!("[ERROR] >>> Entering get_week_day(y={},m={},d={})", y, m, d);
    let day = doomsday(y, m, d);
	eprintln!("[ERROR] get_week_day(y={},m={},d={}): day → {}", y, m, d, day);
    let day_str = match day {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Unknown",
    };

	eprintln!("[WARN] <<< Exiting get_week_day(y={},m={},d={})", y, m, d);
    day_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doomsday_test() {
		eprintln!("[ERROR] >>> Entering doomsday_test()");
        assert_eq!(get_week_day(1990, 3, 21), "Wednesday");
        assert_eq!(get_week_day(2000, 8, 24), "Thursday");
        assert_eq!(get_week_day(2000, 10, 13), "Friday");
        assert_eq!(get_week_day(2001, 4, 18), "Wednesday");
		eprintln!("[WARN] <<< Exiting doomsday_test()");
        assert_eq!(get_week_day(2002, 3, 19), "Tuesday");
    }
}
