#![allow(dead_code)]
use std::ops::Deref;

// A Box that holds a string that will always be surrounded by double quotes (when properly constructed)
struct StringWithQuotes {
    my_str: Box<String>,
}

impl StringWithQuotes {
    fn new(my_str: String) -> Self {
        let mut s = String::new();
        s.push('"');
        s.push_str(&my_str);
        s.push('"');
        StringWithQuotes {
            my_str: Box::new(s),
        }
    }
}

// Helper function used to test deref coercion in tests
fn format_with_greeting(my_str: &str) -> String {
    format!("Greetings, {my_str}")
}

// A node on a linked list, which has a value T or optionally a Box with another Node<T>
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// The head of a linked list
struct List<T> {
    head: Option<Box<Node<T>>>,
}

// ** START EDITS HERE **

// Part 1: Implement the Deref trait on StringWithQuotes (return a reference to the Box holding the String)
impl Deref for StringWithQuotes {
    type Target = Box<String>;
    fn deref(&self) -> &Self::Target {
        &self.my_str
    }
}

// Part 2: Create a consuming iterator for List
// Note: Typically we'd create a separate type that acts as a dedicated Iterable type instead of implementing
// directly on List, but we're doing it this way for simplicity's sake
impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // take() gives us the current value of head (if it is Some), and puts None in its place
        // map() allows us to convert Some(T) to Some(U) (where T = self.head, U = self.head.value)
        self.head.take().map(|node| {
            // We now have ownership of node! (If it exists)
            // Step one forward in the list
            self.head = node.next;
            // Return the current value of node (through deref coercion, we can do .value)
            node.value
        })
    }
}

// ** END EDITS HERE **

#[cfg(test)]
mod tests {
    use super::StringWithQuotes;
    use super::{List, Node};

    #[test]
    fn format_with_greeting() {
        let heap_string = StringWithQuotes::new("Alice".to_string());
        // Even though StringWithQuotes is a Box<String>, deref coercion allows us to treat it like a &str
        let formatted_string = super::format_with_greeting(&heap_string);
        assert_eq!(formatted_string, "Greetings, \"Alice\"");
    }

    #[test]
    fn iterate_through_linked_list() {
        let head = Node {
            value: 1,
            next: Some(Box::new(Node {
                value: 2,
                next: Some(Box::new(Node {
                    value: 3,
                    next: None,
                })),
            })),
        };
        let mut list = List {
            head: Some(Box::new(head)),
        };

        let mut idx = 1;
        while let Some(value) = list.next() {
            assert_eq!(idx, value);
            idx += 1;
        }
    }
}
