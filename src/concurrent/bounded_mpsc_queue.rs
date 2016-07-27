extern crate core;

use std::sync::atomic::AtomicPtr;
use self::core::ptr;

struct Node<T> {
    pub value: Option<T>,
    pub next: AtomicPtr<Node<T>>,
    pub count: usize
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value: Some(value),
            next: Node::null_ptr(),
            count: 0
        }
    }

    fn empty() -> Node<T> {
        Node {
            value: None,
            next: Node::null_ptr(),
            count: 0
        }
    }

    fn null_ptr() -> AtomicPtr<Node<T>> {
        AtomicPtr::new(ptr::null_mut::<Node<T>>())
    }
}

pub struct BoundedMpscQueue<T> {
    _head: AtomicPtr<Node<T>>,
    _tail: AtomicPtr<Node<T>>
}

impl<T> BoundedMpscQueue<T> {
    fn new(capacity: usize) -> BoundedMpscQueue<T> {
        let empty = Node::empty();

        BoundedMpscQueue {
            _head: Node::null_ptr(),
            _tail: Node::null_ptr()
        }
    }
}