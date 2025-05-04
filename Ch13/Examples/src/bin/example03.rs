use std::thread;

fn main() {
    let numbers = vec![2, 4, 6, 8, 10];

    let handles: Vec<_> = numbers
        .into_iter()
        .map(|num| {
            thread::spawn(move || {
                println!("Calculating square of {}", num);
                num * num
            })
        })
        .collect();

    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();

    println!("Squares: {:?}", results);
}
