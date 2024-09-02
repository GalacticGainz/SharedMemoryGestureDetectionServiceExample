/// Represents a point in 2D space.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    // Constructor
    pub(crate) fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Subtract two points
    pub(crate) fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }

    // Add two points
    pub(crate) fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    // Scale a point by a scalar
    pub(crate) fn mul(self, scalar: f64) -> Point {
        Point::new(self.x * scalar, self.y * scalar)
    }

    pub(crate) fn rotate(&self, origin: &Point, degrees: f64) -> Point {
        let radians = degrees.to_radians();
        let x = origin.x + (self.x - origin.x) * radians.cos() - (self.y - origin.y) * radians.sin();
        let y = origin.y + (self.x - origin.x) * radians.sin() + (self.y - origin.y) * radians.cos();
        Point::new(x, y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl std::hash::Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_i64(self.x.to_bits() as i64);
        state.write_i64(self.y.to_bits() as i64);
    }
}