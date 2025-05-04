struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Cleaning up resource: {}", self.name);
    }
}

fn main() {
    let _res1 = Resource { name: String::from("File1") };
    let _res2 = Resource { name: String::from("Database Connection") };
    println!("Resources created");
}
