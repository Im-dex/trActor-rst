struct Node<'T, T: 'T> {
    pub value: &'T mut T
}

pub struct BoundedMpscQueue<T> {

}