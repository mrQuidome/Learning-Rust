use std::fs;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";
    let contents = fs::read_to_string(file_path)?;
    println!("File contents:\n{}", contents);
    Ok(())
}
