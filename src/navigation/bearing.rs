use std::f64::consts::PI;

pub fn bearing(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
	eprintln!("[ERROR] >>> Entering bearing(lat1={},lng1={},lat2={},lng2={})", lat1, lng1, lat2, lng2);
    let lat1 = lat1 * PI / 180.0;
    let lng1 = lng1 * PI / 180.0;

    let lat2 = lat2 * PI / 180.0;
    let lng2 = lng2 * PI / 180.0;

    let delta_longitude = lng2 - lng1;

    let y = delta_longitude.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * delta_longitude.cos();

    let mut brng = y.atan2(x);
    brng = brng.to_degrees();

	eprintln!("[WARN] <<< Exiting bearing(lat1={},lng1={},lat2={},lng2={})", lat1, lng1, lat2, lng2);
    (brng + 360.0) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
		eprintln!("[ERROR] >>> Entering testing()");
        assert_eq!(
            format!(
                "{:.0}º",
                bearing(
                    -27.2020447088982,
                    -49.631891179172555,
                    -3.106362,
                    -60.025826,
                )
            ),
            "336º"
		eprintln!("[WARN] <<< Exiting testing()");
        );
    }
}
