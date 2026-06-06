struct Node<T> {
    score: T,
    id: u64,
    height: u32,
    size: u32,
    balance: i32,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}
