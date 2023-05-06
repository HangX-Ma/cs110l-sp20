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
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Node {value: value, next: next}
    }
}

// implement clone/eq for Node and propagate it to the whole LinkedList
impl<T> Clone for Node<T> where T: Clone {
    fn clone(&self) -> Self {
        Node::new(self.value.clone(), self.next.clone())
    }
}

impl<T> PartialEq for Node<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next
    }
    
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {head: None, size: 0}
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
                },
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

impl<T> Clone for LinkedList<T> where T: Clone {
    fn clone(&self) -> Self {
        LinkedList { head: self.head.clone(), size: self.size.clone() }
    }
}

impl<T> PartialEq for LinkedList<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.head == other.head
    }
}

pub struct LinkedListIterator<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<T: Clone> Iterator for LinkedListIterator<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value.clone())
            },
            None => None
        }
    }
}

impl<'a, T: Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { current: &self.head }
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for LinkedList<f64> {
    fn compute_norm(&self) -> f64 {
        self.into_iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}