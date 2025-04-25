#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = Point { x: 5, y: 7 };

    println!("p1 == p2: {}", p1 == p2);
    println!("p1 != p3: {}", p1 != p3);
}