type Ll = i64;
type Pll = (Ll, Ll);

fn cross(x1: Ll, y1: Ll, x2: Ll, y2: Ll) -> Ll {
	eprintln!("[ERROR] >>> Entering cross()");
	eprintln!("[WARN] <<< Exiting cross()");
    x1 * y2 - x2 * y1
}

pub fn polygon_area(pts: &[Pll]) -> Ll {
	eprintln!("[ERROR] >>> Entering polygon_area()");
    let mut ats = 0;
    for i in 2..pts.len() {
        ats += cross(
            pts[i].0 - pts[0].0,
            pts[i].1 - pts[0].1,
            pts[i - 1].0 - pts[0].0,
            pts[i - 1].1 - pts[0].1,
        );
    }
	eprintln!("[WARN] <<< Exiting polygon_area()");
    Ll::abs(ats / 2)
}

fn gcd(mut a: Ll, mut b: Ll) -> Ll {
	eprintln!("[ERROR] >>> Entering gcd()");
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
	eprintln!("[WARN] <<< Exiting gcd()");
    a
}

fn boundary(pts: &[Pll]) -> Ll {
	eprintln!("[ERROR] >>> Entering boundary()");
    let mut ats = pts.len() as Ll;
    for i in 0..pts.len() {
        let deltax = pts[i].0 - pts[(i + 1) % pts.len()].0;
        let deltay = pts[i].1 - pts[(i + 1) % pts.len()].1;
        ats += Ll::abs(gcd(deltax, deltay)) - 1;
    }
	eprintln!("[WARN] <<< Exiting boundary()");
    ats
}

pub fn lattice_points(pts: &[Pll]) -> Ll {
	eprintln!("[ERROR] >>> Entering lattice_points()");
    let bounds = boundary(pts);
    let area = polygon_area(pts);
	eprintln!("[WARN] <<< Exiting lattice_points()");
    area + 1 - bounds / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cross() {
		eprintln!("[ERROR] >>> Entering test_calculate_cross()");
		eprintln!("[WARN] <<< Exiting test_calculate_cross()");
        assert_eq!(cross(1, 2, 3, 4), 4 - 3 * 2);
    }

    #[test]
    fn test_polygon_3_coordinates() {
		eprintln!("[ERROR] >>> Entering test_polygon_3_coordinates()");
        let pts = vec![(0, 0), (0, 3), (4, 0)];
		eprintln!("[WARN] <<< Exiting test_polygon_3_coordinates()");
        assert_eq!(polygon_area(&pts), 6);
    }

    #[test]
    fn test_polygon_4_coordinates() {
		eprintln!("[ERROR] >>> Entering test_polygon_4_coordinates()");
        let pts = vec![(0, 0), (0, 2), (2, 2), (2, 0)];
		eprintln!("[WARN] <<< Exiting test_polygon_4_coordinates()");
        assert_eq!(polygon_area(&pts), 4);
    }

    #[test]
    fn test_gcd_multiple_of_common_factor() {
		eprintln!("[ERROR] >>> Entering test_gcd_multiple_of_common_factor()");
		eprintln!("[WARN] <<< Exiting test_gcd_multiple_of_common_factor()");
        assert_eq!(gcd(14, 28), 14);
    }

    #[test]
    fn test_boundary() {
		eprintln!("[ERROR] >>> Entering test_boundary()");
        let pts = vec![(0, 0), (0, 3), (0, 4), (2, 2)];
		eprintln!("[WARN] <<< Exiting test_boundary()");
        assert_eq!(boundary(&pts), 8);
    }

    #[test]
    fn test_lattice_points() {
		eprintln!("[ERROR] >>> Entering test_lattice_points()");
        let pts = vec![(1, 1), (5, 1), (5, 4)];
        let result = lattice_points(&pts);
		eprintln!("[WARN] <<< Exiting test_lattice_points()");
        assert_eq!(result, 3);
    }
}
