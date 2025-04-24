struct Counter {
    count: i32,
}

impl Counter {
    fn print_and_consume(self) {
        println!("print and consume: {}", self.count);
    }

    fn get_count(&self) -> i32 {
        println!("Current count is: {}", self.count);
        self.count
    }

    fn increment(&mut self) {
        self.count += 1;
        println!("Incremented count to: {}", self.count);
    }
}

fn main() {
    let mut counter = Counter { count: 0 };

    counter.increment();
    counter.increment();

    let current_count = counter.get_count();
    println!("{}", current_count);

    counter.print_and_consume();
}
