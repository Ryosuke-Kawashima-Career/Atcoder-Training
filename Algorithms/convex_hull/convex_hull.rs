#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

struct ConvexHull {
    points: Vec<Point>
}

impl ConvexHull {
    fn new(coord: &Vec<(i64, i64)>) -> ConvexHull {
        let mut points: Vec<Point> = Vec::new();
        for &(x, y) in coord.iter() {
            points.push(Point {x, y});
        }
        points.sort_by_key(|point| (point.0, point.1));
        Self {
            points
        }
    }

    fn cross_product(p0: Point, p1: Point, p2: Point) -> i64 {
        let x_diff1: i64 = p1.x - p0.x;
        let x_diff2: i64 = p2.x - p0.x;
        let y_diff1: i64 = p1.y - p0.y;
        let y_diff2: i64 = p2.y - p0.y;
        return x_diff1 * y_diff2 - x_diff2 * y_diff1;
    }

    fn monotone_chain(self) -> Vec<Point> {
        let mut lower: Vec<Point> = Vec::new();
        let mut upper: Vec<Point> = Vec::new();
        for &point in self.points.iter() {
            if lower.len() >= 2 && 
            Self::cross_product(self.points[self.points.len()-2], self.points[self.points.len()-1], point) <= 0 {
                lower.pop()
            }
            self.points.push(point);
        }
        for &point in self.points.iter().rev() {
            if upper.len() >= 2 && 
            Self::cross_product(self.points[self.points.len()-2], self.points[self.points.len()-1], point) <= 0 {
                upper.pop()
            }
            self.points.push(point);
        }
        lower.pop();
        upper.pop();
        lower.extend(upper);
        lower
    }
}