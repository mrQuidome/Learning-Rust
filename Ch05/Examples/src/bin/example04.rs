trait Describable {
    fn describe(&self) -> String {
        String::from("An object")
    }
}

struct Book {
    title: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("A book titled '{}'", self.title)
    }
}

struct Author;

impl Describable for Author {
    // Uses the default implementation of describe
}

fn main() {}