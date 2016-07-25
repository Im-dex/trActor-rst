use std::ops::AddAssign;

pub trait MessageQueue<T> {
    fn is_empty(&self) -> bool;
    fn non_empty(&self) -> bool;

    // single consumer warranty
    fn deque(&mut self) -> T;

    // single consumer warranty
    fn peek(&self) -> T;

    // single consumer warranty
    fn clear(&mut self);

    // TODO: move all elements
}

trait BoundedMessageQueue<T> : MessageQueue<T> + AddAssign<T> {
    fn enqueue(&mut self, value: T) -> bool;

    fn add_assign(&mut self, value: T) -> bool { self.enqueue(value) }
}

trait UnboundedMessageQueue<T> : MessageQueue<T> + AddAssign<T> {
    fn enqueue(&mut self, value: T);
    fn add_assign(&mut self, value: T) { self.enqueue(value) }
}