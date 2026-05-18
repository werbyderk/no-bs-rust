#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

trait Consumer {
    fn consume(&self, value: usize);
}

// Don't worry about the `dyn` keyword too much at this point.
// We're telling Rust to make a Vector of Rc that holds Trait Objects
// where each element implements the trait `Consumer`.
// We'll get into Trait Objects in a future video
struct Publisher {
    consumers: Vec<Rc<dyn Consumer>>,
}

impl Publisher {
    fn new(consumers: Vec<Rc<dyn Consumer>>) -> Self {
        Publisher { consumers }
    }
    fn publish(&self) {
        for (i, c) in self.consumers.iter().enumerate() {
            c.consume(i);
        }
    }
}

// ** START EDITS HERE **

// Define contents. We want to send all consumed values to a Vec<usize>
struct MockConsumer {
    contents: RefCell<Vec<usize>>,
}

impl MockConsumer {
    fn new() -> Self {
        // Create a new MockConsumer and initialize contents
        MockConsumer {
            contents: RefCell::new(vec![]),
        }
    }
}

impl Consumer for MockConsumer {
    // Implement the Consumer trait for MockConsumer.
    // We should be pushing `value` to `contents`
    fn consume(&self, value: usize) {
        self.contents.borrow_mut().push(value);
    }
}

// ** END EDITS HERE **

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::{Consumer, MockConsumer, Publisher};

    #[test]
    fn pub_sub() {
        // We're using `Rc` here so we can pass strong references of the consumers
        // to the Publisher, and still compare the consumers's contents later in the test
        let consumer_1 = Rc::new(MockConsumer::new());
        let consumer_2 = Rc::new(MockConsumer::new());
        // By using `dyn`, we're telling Rust to create a Vector of Trait Objects,
        // something we'll get to in a future video
        let consumers: Vec<Rc<dyn Consumer>> = vec![consumer_1.clone(), consumer_2.clone()];
        let p = Publisher::new(consumers);

        p.publish();
        p.publish();

        let expected_1: Vec<usize> = vec![0, 0];
        let expected_2: Vec<usize> = vec![1, 1];

        // We use `as_slice()` to allow comparison between contents and expected contents.
        // This is a rare case where dereference coercion can't be done automatically.
        // Try removing the `as_slice` method and see what kind of compilation errors you get
        assert_eq!(consumer_1.contents.borrow().as_slice(), &expected_1);
        assert_eq!(consumer_2.contents.borrow().as_slice(), &expected_2);
    }
}
