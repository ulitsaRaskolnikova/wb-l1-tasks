#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    // Метод для вычисления расстояния до другой точки
    fn distance(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let point1 = Point::new(1.0, 2.0);
    let point2 = Point::new(4.0, 6.0);

    let distance = point1.distance(&point2);
    println!(
        "The distance between {:?} and {:?} is {:.2}",
        point1, point2, distance
    );
}
