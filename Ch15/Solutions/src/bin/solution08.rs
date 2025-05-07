union IntOrFloat {
    int: u32,
    float: f32,
}

fn main() {
    let value = IntOrFloat { int: 42 };

    unsafe {
        // Reading from a different field than was written
        println!("Stored as int: {}", value.int);
        println!("Interpreted as float: {}", value.float);
    }
}
