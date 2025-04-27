use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_path = "example.txt";
    let file_result = File::open(file_path);

    match file_result {
        Ok(_) => println!("File opened successfully: {:?}", file_path),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("File not found, creating a new file...");
            match File::create("example.txt") {
                Ok(_) => println!("File created successfully."),
                Err(e) => println!("Failed to create the file: {:?}", e),
            }
        }
        Err(error) => println!("Failed to open the file: {:?}", error),
    }
}
