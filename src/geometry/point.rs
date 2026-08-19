use std::ops::Sub;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
		log::debug!(">>> Entering new(x={},y={})", x, y);
        Point { x, y }
    }

    // Returns the orientation of consecutive segments ab and bc.
    pub fn consecutive_orientation(&self, b: &Point, c: &Point) -> f64 {
		log::debug!(">>> Entering consecutive_orientation(b={:?},c={:?})", b, c);
        let p1 = b - self;
        let p2 = c - self;
		log::debug!("<<< Exiting consecutive_orientation(b={:?},c={:?})", b, c);
        p1.cross_prod(&p2)
    }

    pub fn cross_prod(&self, other: &Point) -> f64 {
		log::debug!(">>> Entering cross_prod(other={:?})", other);
        self.x * other.y - self.y * other.x
    }

    pub fn euclidean_distance(&self, other: &Point) -> f64 {
		log::debug!(">>> Entering euclidean_distance(other={:?})", other);
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
		log::debug!(">>> Entering sub()");
        let x = self.x - other.x;
        let y = self.y - other.y;
		log::debug!("<<< Exiting sub()");
        Point::new(x, y)
    }
}
