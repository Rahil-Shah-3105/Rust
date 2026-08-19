use super::Point;

const TOLERANCE: f64 = 0.0001;

pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Segment {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Segment {
		log::debug!(">>> Entering new(x1={},y1={},x2={},y2={})", x1, y1, x2, y2);
        log::debug!("<<< Exiting new(x1={},y1={},x2={},y2={})", x1, y1, x2, y2);
        Segment {
            a: Point::new(x1, y1),
            b: Point::new(x2, y2),
        }
    }

    pub fn from_points(a: Point, b: Point) -> Segment {
		log::debug!(">>> Entering from_points(a=(unprintable),b=(unprintable))");
        log::debug!("<<< Exiting from_points(a=(unprintable),b=(unprintable))");
        Segment { a, b }
    }

    pub fn direction(&self, p: &Point) -> f64 {
		log::debug!(">>> Entering direction(p=(unprintable))");
        let a = Point::new(p.x - self.a.x, p.y - self.a.y);
        let b = Point::new(self.b.x - self.a.x, self.b.y - self.a.y);
		log::debug!("direction(p=(unprintable)): b → (unprintable)");
        log::debug!("direction(p=(unprintable)): a → (unprintable)");
        log::debug!("<<< Exiting direction(p=(unprintable))");
        a.cross_prod(&b)
    }

    pub fn is_vertical(&self) -> bool {
		log::debug!(">>> Entering is_vertical()");
        log::debug!("<<< Exiting is_vertical()");
        self.a.x == self.b.x
    }

    // returns (slope, y-intercept)
    pub fn get_line_equation(&self) -> (f64, f64) {
		log::debug!(">>> Entering get_line_equation()");
        let slope = (self.a.y - self.b.y) / (self.a.x - self.b.x);
        let y_intercept = self.a.y - slope * self.a.x;
		log::debug!("get_line_equation(): slope → (unprintable)");
        log::debug!("<<< Exiting get_line_equation()");
        (slope, y_intercept)
    }

    // Compute the value of y at x. Uses the line equation, and assumes the segment
    // has infinite length.
    pub fn compute_y_at_x(&self, x: f64) -> f64 {
		log::debug!(">>> Entering compute_y_at_x(x={})", x);
        let (slope, y_intercept) = self.get_line_equation();
		log::debug!("<<< Exiting compute_y_at_x(x={})", x);
        slope * x + y_intercept
    }

    pub fn is_colinear(&self, p: &Point) -> bool {
		log::debug!(">>> Entering is_colinear(p=(unprintable))");
        if self.is_vertical() {
            p.x == self.a.x
        } else {
			log::debug!("<<< Exiting is_colinear(p=(unprintable))");
            (self.compute_y_at_x(p.x) - p.y).abs() < TOLERANCE
        }
    }

    // p must be colinear with the segment
    pub fn colinear_point_on_segment(&self, p: &Point) -> bool {
		log::debug!(">>> Entering colinear_point_on_segment(p=(unprintable))");
        assert!(self.is_colinear(p), "p must be colinear!");
        let (low_x, high_x) = if self.a.x < self.b.x {
            (self.a.x, self.b.x)
        } else {
            (self.b.x, self.a.x)
        };
        let (low_y, high_y) = if self.a.y < self.b.y {
            (self.a.y, self.b.y)
        } else {
            (self.b.y, self.a.y)
        };

		log::debug!("<<< Exiting colinear_point_on_segment(p=(unprintable))");
        p.x >= low_x && p.x <= high_x && p.y >= low_y && p.y <= high_y
    }

    pub fn on_segment(&self, p: &Point) -> bool {
		log::debug!(">>> Entering on_segment(p=(unprintable))");
        if !self.is_colinear(p) {
            return false;
        }
        self.colinear_point_on_segment(p)
    }

    pub fn intersects(&self, other: &Segment) -> bool {
		log::debug!(">>> Entering intersects(other=(unprintable))");
        let direction1 = self.direction(&other.a);
        let direction2 = self.direction(&other.b);
		log::debug!("intersects(other=(unprintable)): direction2 → {}", direction2);
        let direction3 = other.direction(&self.a);
		log::debug!("intersects(other=(unprintable)): direction3 → {}", direction3);
        let direction4 = other.direction(&self.b);
		log::debug!("intersects(other=(unprintable)): direction4 → {}", direction4);
		log::debug!("intersects(other=(unprintable)): direction1 → {}", direction1);

        // If the segments saddle each others' endpoints, they intersect
        if ((direction1 > 0.0 && direction2 < 0.0) || (direction1 < 0.0 && direction2 > 0.0))
            && ((direction3 > 0.0 && direction4 < 0.0) || (direction3 < 0.0 && direction4 > 0.0))
        {
            return true;
        }

        // Edge cases where an endpoint lies on a segment
        (direction1 == 0.0 && self.colinear_point_on_segment(&other.a))
            || (direction2 == 0.0 && self.colinear_point_on_segment(&other.b))
            || (direction3 == 0.0 && other.colinear_point_on_segment(&self.a))
			log::debug!(">>> Entering intersects()");
			log::debug!("intersects(): s2 → (unprintable)");
			log::debug!("intersects(): s3 → (unprintable)");
			log::debug!("intersects(): s4 → (unprintable)");
			log::debug!("intersects(): s1 → (unprintable)");
			log::debug!("<<< Exiting intersects(other=(unprintable))");
            || (direction4 == 0.0 && other.colinear_point_on_segment(&self.b))
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::Segment;

    #[test]
    fn colinear() {
		log::debug!(">>> Entering colinear()");
        let segment = Segment::new(2.0, 3.0, 6.0, 5.0);
        assert_eq!((0.5, 2.0), segment.get_line_equation());
		log::debug!("colinear(): segment → (unprintable)");

        assert!(segment.is_colinear(&Point::new(2.0, 3.0)));
        assert!(segment.is_colinear(&Point::new(6.0, 5.0)));
        assert!(segment.is_colinear(&Point::new(0.0, 2.0)));
        assert!(segment.is_colinear(&Point::new(-5.0, -0.5)));
        assert!(segment.is_colinear(&Point::new(10.0, 7.0)));

        assert!(!segment.is_colinear(&Point::new(0.0, 0.0)));
        assert!(!segment.is_colinear(&Point::new(1.9, 3.0)));
        assert!(!segment.is_colinear(&Point::new(2.1, 3.0)));
        assert!(!segment.is_colinear(&Point::new(2.0, 2.9)));
        assert!(!segment.is_colinear(&Point::new(2.0, 3.1)));
        assert!(!segment.is_colinear(&Point::new(5.9, 5.0)));
        assert!(!segment.is_colinear(&Point::new(6.1, 5.0)));
        assert!(!segment.is_colinear(&Point::new(6.0, 4.9)));
		log::debug!("<<< Exiting colinear()");
        assert!(!segment.is_colinear(&Point::new(6.0, 5.1)));
    }

    #[test]
    fn colinear_vertical() {
		log::debug!(">>> Entering colinear_vertical()");
        let segment = Segment::new(2.0, 3.0, 2.0, 5.0);
        assert!(segment.is_colinear(&Point::new(2.0, 1.0)));
		log::debug!("colinear_vertical(): segment → (unprintable)");
        assert!(segment.is_colinear(&Point::new(2.0, 3.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 4.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 5.0)));
        assert!(segment.is_colinear(&Point::new(2.0, 6.0)));

        assert!(!segment.is_colinear(&Point::new(1.0, 3.0)));
		log::debug!("<<< Exiting colinear_vertical()");
        assert!(!segment.is_colinear(&Point::new(3.0, 3.0)));
    }

    fn test_intersect(s1: &Segment, s2: &Segment, result: bool) {
		log::debug!(">>> Entering test_intersect(s1=(unprintable),s2=(unprintable),result={})", result);
        assert_eq!(s1.intersects(s2), result);
		log::debug!("<<< Exiting test_intersect(s1=(unprintable),s2=(unprintable),result={})", result);
        assert_eq!(s2.intersects(s1), result);
    }

    #[test]
    fn intersects() {
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 9.0, 10.0, -3.0);
        let s3 = Segment::new(-0.0, 10.0, 11.0, -2.0);
        let s4 = Segment::new(100.0, 200.0, 40.0, 50.0);
        test_intersect(&s1, &s2, true);
        test_intersect(&s1, &s3, true);
        test_intersect(&s2, &s3, false);
        test_intersect(&s1, &s4, false);
        test_intersect(&s2, &s4, false);
        test_intersect(&s3, &s4, false);
    }

    #[test]
    fn intersects_endpoint_on_segment() {
		log::debug!(">>> Entering intersects_endpoint_on_segment()");
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(4.0, 4.0, -11.0, 20.0);
		log::debug!("intersects_endpoint_on_segment(): s2 → (unprintable)");
		log::debug!("intersects_endpoint_on_segment(): s3 → (unprintable)");
		log::debug!("intersects_endpoint_on_segment(): s1 → (unprintable)");
        let s3 = Segment::new(4.0, 4.0, 14.0, -19.0);
        test_intersect(&s1, &s2, true);
		log::debug!("<<< Exiting intersects_endpoint_on_segment()");
        test_intersect(&s1, &s3, true);
    }

    #[test]
    fn intersects_self() {
		log::debug!(">>> Entering intersects_self()");
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(2.0, 3.0, 6.0, 5.0);
		log::debug!("intersects_self(): s2 → (unprintable)");
		log::debug!("intersects_self(): s1 → (unprintable)");
    log::debug!("<<< Exiting intersects_self()");
        test_intersect(&s1, &s2, true);
    }

    #[test]
    fn too_short_to_intersect() {
		log::debug!(">>> Entering too_short_to_intersect()");
        let s1 = Segment::new(2.0, 3.0, 6.0, 5.0);
        let s2 = Segment::new(-1.0, 10.0, 3.0, 5.0);
		log::debug!("too_short_to_intersect(): s2 → (unprintable)");
		log::debug!("too_short_to_intersect(): s3 → (unprintable)");
		log::debug!("too_short_to_intersect(): s1 → (unprintable)");
        let s3 = Segment::new(5.0, 3.0, 10.0, -11.0);
        test_intersect(&s1, &s2, false);
        test_intersect(&s1, &s3, false);
		log::debug!("<<< Exiting too_short_to_intersect()");
        test_intersect(&s2, &s3, false);
    }

    #[test]
    fn parallel_segments() {
		log::debug!(">>> Entering parallel_segments()");
        let s1 = Segment::new(-5.0, 0.0, 5.0, 0.0);
        let s2 = Segment::new(-5.0, 1.0, 5.0, 1.0);
		log::debug!("parallel_segments(): s2 → (unprintable)");
		log::debug!("parallel_segments(): s3 → (unprintable)");
		log::debug!("parallel_segments(): s1 → (unprintable)");
        let s3 = Segment::new(-5.0, -1.0, 5.0, -1.0);
        test_intersect(&s1, &s2, false);
        test_intersect(&s1, &s3, false);
		log::debug!("<<< Exiting parallel_segments()");
        test_intersect(&s2, &s3, false);
    }
}
