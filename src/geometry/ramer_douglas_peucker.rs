use crate::geometry::Point;

pub fn ramer_douglas_peucker(points: &[Point], epsilon: f64) -> Vec<Point> {
	log::debug!(">>> Entering ramer_douglas_peucker(points=(unprintable),epsilon={})", epsilon);
    if points.len() < 3 {
        return points.to_vec();
    }
    let mut dmax = 0.0;
	log::debug!("ramer_douglas_peucker(points=(unprintable),epsilon={}): dmax → {}", epsilon, dmax);
    let mut index = 0;
    let end = points.len() - 1;
	log::debug!("ramer_douglas_peucker(points=(unprintable),epsilon={}): end → (unprintable)", epsilon);

    for i in 1..end {
        let d = perpendicular_distance(&points[i], &points[0], &points[end]);
		log::debug!("ramer_douglas_peucker(points=(unprintable),epsilon={}): d → {}", epsilon, d);
        if d > dmax {
            index = i;
            dmax = d;
        }
    }

    if dmax > epsilon {
        let mut results = ramer_douglas_peucker(&points[..=index], epsilon);
		log::debug!("ramer_douglas_peucker(points=(unprintable),epsilon={}): results → (unprintable)", epsilon);
        results.pop();
        results.extend(ramer_douglas_peucker(&points[index..], epsilon));
        results
    } else {
        vec![points[0].clone(), points[end].clone()]
	log::debug!("<<< Exiting ramer_douglas_peucker(points=(unprintable),epsilon={})", epsilon);
    }
}

fn perpendicular_distance(p: &Point, a: &Point, b: &Point) -> f64 {
	log::debug!(">>> Entering perpendicular_distance(p=(unprintable),a=(unprintable),b=(unprintable))");
    let num = (b.y - a.y) * p.x - (b.x - a.x) * p.y + b.x * a.y - b.y * a.x;
    let den = a.euclidean_distance(b);
	log::debug!("perpendicular_distance(p=(unprintable),a=(unprintable),b=(unprintable)): den → (unprintable)");
    log::debug!("perpendicular_distance(p=(unprintable),a=(unprintable),b=(unprintable)): num → (unprintable)");
    log::debug!("<<< Exiting perpendicular_distance(p=(unprintable),a=(unprintable),b=(unprintable))");
    num.abs() / den
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_perpendicular_distance {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (p, a, b, expected) = $test_case;
                    assert_eq!(perpendicular_distance(&p, &a, &b), expected);
                    assert_eq!(perpendicular_distance(&p, &b, &a), expected);
                }
            )*
        };
    }

    test_perpendicular_distance! {
        basic: (Point::new(4.0, 0.0), Point::new(0.0, 0.0), Point::new(0.0, 3.0), 4.0),
        basic_shifted_1: (Point::new(4.0, 1.0), Point::new(0.0, 1.0), Point::new(0.0, 4.0), 4.0),
        basic_shifted_2: (Point::new(2.0, 1.0), Point::new(-2.0, 1.0), Point::new(-2.0, 4.0), 4.0),
    }

    #[test]
    fn test_ramer_douglas_peucker_polygon() {
		log::debug!(">>> Entering test_ramer_douglas_peucker_polygon()");
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
		log::debug!("test_ramer_douglas_peucker_polygon(): b → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygon(): c → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygon(): d → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygon(): e → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygon(): f → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygon(): g → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygon(): h → (unprintable)");
        log::debug!("test_ramer_douglas_peucker_polygon(): a → (unprintable)");
        let c = Point::new(2.0, 0.0);
        let d = Point::new(2.0, 1.0);
        let e = Point::new(2.0, 2.0);
        let f = Point::new(1.0, 2.0);
        let g = Point::new(0.0, 2.0);
        let h = Point::new(0.0, 1.0);
        let polygon = vec![
            a.clone(),
            b,
            c.clone(),
            d,
            e.clone(),
            f,
            g.clone(),
            h.clone(),
        ];
        let epsilon = 0.7;
        let result = ramer_douglas_peucker(&polygon, epsilon);
		log::debug!("test_ramer_douglas_peucker_polygon(): result → (unprintable)");
		log::debug!("<<< Exiting test_ramer_douglas_peucker_polygon()");
        assert_eq!(result, vec![a, c, e, g, h]);
    }

    #[test]
    fn test_ramer_douglas_peucker_polygonal_chain() {
		log::debug!(">>> Entering test_ramer_douglas_peucker_polygonal_chain()");
        let a = Point::new(0., 0.);
        let b = Point::new(2., 0.5);
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): b → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): c → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): d → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): e → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): a → (unprintable)");
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): points → (unprintable)");
        let c = Point::new(3., 3.);
        let d = Point::new(6., 3.);
        let e = Point::new(8., 4.);

        let points = vec![a.clone(), b, c, d, e.clone()];

        let epsilon = 3.; // The epsilon is quite large, so the result will be a single line
        let result = ramer_douglas_peucker(&points, epsilon);
		log::debug!("test_ramer_douglas_peucker_polygonal_chain(): result → (unprintable)");
		log::debug!("<<< Exiting test_ramer_douglas_peucker_polygonal_chain()");
        assert_eq!(result, vec![a, e]);
    }

    #[test]
    fn test_less_than_three_points() {
		log::debug!(">>> Entering test_less_than_three_points()");
        let a = Point::new(0., 0.);
        let b = Point::new(1., 1.);
		log::debug!("test_less_than_three_points(): b → (unprintable)");
		log::debug!("test_less_than_three_points(): a → (unprintable)");

        let epsilon = 0.1;

        assert_eq!(ramer_douglas_peucker(&[], epsilon), vec![]);
        assert_eq!(
            ramer_douglas_peucker(std::slice::from_ref(&a), epsilon),
            vec![a.clone()]
        );
        assert_eq!(
            ramer_douglas_peucker(&[a.clone(), b.clone()], epsilon),
            vec![a, b]
		log::debug!("<<< Exiting test_less_than_three_points()");
        );
    }
}
