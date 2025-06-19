use std::cell::RefCell;

struct Logger {
    messages: RefCell<Vec<String>>,
}

impl Logger {
    fn log(&self, message: String) {
        self.messages.borrow_mut().push(message);
    }

    fn show_logs(&self) {
        for msg in self.messages.borrow().iter() {
            println!("{}", msg);
        }
    }
}

fn main() {
    let logger = Logger {
        messages: RefCell::new(Vec::new()),
    };

    logger.log("First message".to_string());
    logger.log("Second message".to_string());

    logger.show_logs();
}
