type Ll = i64;
type Pll = (Ll, Ll);

fn cross(x1: Ll, y1: Ll, x2: Ll, y2: Ll) -> Ll {
	log::debug!(">>> Entering cross(x1=(unprintable),y1=(unprintable),x2=(unprintable),y2=(unprintable))");
    log::debug!("<<< Exiting cross(x1=(unprintable),y1=(unprintable),x2=(unprintable),y2=(unprintable))");
    x1 * y2 - x2 * y1
}

pub fn polygon_area(pts: &[Pll]) -> Ll {
	log::debug!(">>> Entering polygon_area(pts=(unprintable))");
    let mut ats = 0;
    for i in 2..pts.len() {
        ats += cross(
            pts[i].0 - pts[0].0,
            pts[i].1 - pts[0].1,
            pts[i - 1].0 - pts[0].0,
            pts[i - 1].1 - pts[0].1,
        );
    }
	log::debug!("<<< Exiting polygon_area(pts=(unprintable))");
    Ll::abs(ats / 2)
}

fn gcd(mut a: Ll, mut b: Ll) -> Ll {
	log::debug!(">>> Entering gcd(a=(unprintable),b=(unprintable))");
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
	log::debug!("<<< Exiting gcd(a=(unprintable),b=(unprintable))");
    a
}

fn boundary(pts: &[Pll]) -> Ll {
	log::debug!(">>> Entering boundary(pts=(unprintable))");
    let mut ats = pts.len() as Ll;
    for i in 0..pts.len() {
		log::debug!("boundary(pts=(unprintable)): ats → (unprintable)");
		log::debug!("boundary(pts=(unprintable)): deltax → (unprintable)");
		log::debug!("boundary(pts=(unprintable)): deltay → (unprintable)");
        let deltax = pts[i].0 - pts[(i + 1) % pts.len()].0;
        let deltay = pts[i].1 - pts[(i + 1) % pts.len()].1;
        ats += Ll::abs(gcd(deltax, deltay)) - 1;
    }
	log::debug!("<<< Exiting boundary(pts=(unprintable))");
    ats
}

pub fn lattice_points(pts: &[Pll]) -> Ll {
	log::debug!(">>> Entering lattice_points(pts=(unprintable))");
    let bounds = boundary(pts);
    let area = polygon_area(pts);
	log::debug!("lattice_points(pts=(unprintable)): area → (unprintable)");
    log::debug!("lattice_points(pts=(unprintable)): bounds → (unprintable)");
    log::debug!("<<< Exiting lattice_points(pts=(unprintable))");
    area + 1 - bounds / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cross() {
		log::debug!(">>> Entering test_calculate_cross()");
		log::debug!("<<< Exiting test_calculate_cross()");
        assert_eq!(cross(1, 2, 3, 4), 4 - 3 * 2);
    }

    #[test]
    fn test_polygon_3_coordinates() {
		log::debug!(">>> Entering test_polygon_3_coordinates()");
        let pts = vec![(0, 0), (0, 3), (4, 0)];
		log::debug!("test_polygon_3_coordinates(): pts → (unprintable)");
    log::debug!("<<< Exiting test_polygon_3_coordinates()");
        assert_eq!(polygon_area(&pts), 6);
    }

    #[test]
    fn test_polygon_4_coordinates() {
		log::debug!(">>> Entering test_polygon_4_coordinates()");
        let pts = vec![(0, 0), (0, 2), (2, 2), (2, 0)];
		log::debug!("test_polygon_4_coordinates(): pts → (unprintable)");
    log::debug!("<<< Exiting test_polygon_4_coordinates()");
        assert_eq!(polygon_area(&pts), 4);
    }

    #[test]
    fn test_gcd_multiple_of_common_factor() {
		log::debug!(">>> Entering test_gcd_multiple_of_common_factor()");
		log::debug!("<<< Exiting test_gcd_multiple_of_common_factor()");
        assert_eq!(gcd(14, 28), 14);
    }

    #[test]
    fn test_boundary() {
		log::debug!(">>> Entering test_boundary()");
        let pts = vec![(0, 0), (0, 3), (0, 4), (2, 2)];
		log::debug!("test_boundary(): pts → (unprintable)");
    log::debug!("<<< Exiting test_boundary()");
        assert_eq!(boundary(&pts), 8);
    }

    #[test]
    fn test_lattice_points() {
		log::debug!(">>> Entering test_lattice_points()");
        let pts = vec![(1, 1), (5, 1), (5, 4)];
        let result = lattice_points(&pts);
		log::debug!("test_lattice_points(): result → (unprintable)");
		log::debug!("test_lattice_points(): pts → (unprintable)");
    log::debug!("<<< Exiting test_lattice_points()");
        assert_eq!(result, 3);
    }
}
