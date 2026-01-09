use std::fmt;
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {
            value: value,
            next: next,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> LinkedList<T> {
        // Create a new empty list to hold the cloned nodes
        let mut cloned: LinkedList<T> = LinkedList::new();

        // Iterator for traversing the original list (immutable reference)
        let mut current: &Option<Box<Node<T>>> = &self.head;

        // Mutable reference to the position where we'll insert the next node
        // Initially points to cloned.head, then moves to each node's next field
        let mut tail: &mut Option<Box<Node<T>>> = &mut cloned.head;

        // Traverse the original list
        while let Some(node) = current {
            // Create new node with cloned value and no next pointer (yet)
            *tail = Some(Box::new(Node::new(node.value.clone(), None)));

            // Move tail to point to the next field of the node we just created
            // This prepares us to append the next node in the next iteration
            tail = &mut tail.as_mut().unwrap().next;

            // Move to the next node in the original list
            current = &node.next
        }

        // Return the cloned list (size is 0, but could set cloned.size = self.size)
        cloned.size = self.size;
        cloned
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        let mut current_a: &Option<Box<Node<T>>> = &self.head;
        let mut current_b: &Option<Box<Node<T>>> = &other.head;

        loop {
            match (current_a, current_b) {
                (Some(node_a), Some(node_b)) => {
                    if node_a.value != node_b.value {
                        return false;
                    }
                    current_a = &node_a.next;
                    current_b = &node_b.next;
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<T: Clone> Iterator for LinkedListIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value.clone())
            }
            None => None,
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            current: &self.head,
        }
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.pop_front()
    }
}
