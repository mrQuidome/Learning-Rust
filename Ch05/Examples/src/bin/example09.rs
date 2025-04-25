#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 5, y: 7 };

    let p3 = p1 + p2;

    println!("{:?}", p3);
}
