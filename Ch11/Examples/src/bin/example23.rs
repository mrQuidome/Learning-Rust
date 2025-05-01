fn main() {
    let words = vec!["apple", "banana", "cherry", "date"];

    if let Some(word) = words.iter().find(|&&w| w.len() > 5) {
        println!("Found: {}", word);
    } else {
        println!("No word found with more than 5 characters.");
    }
}
