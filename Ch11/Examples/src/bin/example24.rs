fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    if numbers.iter().any(|&n| n > 3) {
        println!("At least one number is greater than 3.");
    } else {
        println!("No number is greater than 3.");
    }
}
