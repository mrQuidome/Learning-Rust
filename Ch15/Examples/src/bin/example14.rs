fn main() {
    let raw_dangling: *const i32;
    {
        let x = 10;
        raw_dangling = &x;
        unsafe {
            println!("raw_dangling: {:?}", *raw_dangling);
        }
    }

    // polute the stack
    let y1 = 100;

    unsafe {
        println!("raw_dangling: {:?}", *raw_dangling);
    }

    // prevent optimization
    println!("y1: {}", y1);
}