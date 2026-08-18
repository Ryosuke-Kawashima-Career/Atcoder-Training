#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Region {
    site: Point,
    vertices: Vec<Point>,
}

#[derive(Debug, Clone)]
struct Event {
    centroid: Point,
    is_site: bool,
    arc: Option<usize>,
}

impl Ord for Event {
    fn cmp(&self, other) -> bool {
        
    }
}