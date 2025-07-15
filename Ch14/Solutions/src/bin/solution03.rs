// This cannot work it is not possible to return a reference to an owned value
// The only way out is to return String

fn substring_after(text: String, prefix: &str) -> &str {
    text.split_once(prefix).map(|(_, rest)| rest).unwrap_or("")
}

fn main() {
    let text = String::from("Learning Rust");
    let prefix = " ";
    let result = substring_after(text, prefix);

    println!("Substring after '{}': {}", prefix, result);
}
