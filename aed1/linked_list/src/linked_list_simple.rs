use std::ptr::NonNull;

//  Um node simples
//  NonNull diz para o compilador que o ponteiro nunca 
//  para o endereço 0, gerando algumas otimizações 
//  quando colocado dentro de um Option.
pub struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>, 
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        return Node {
            value,
            next: None,
        };
    }

    pub fn next(&self) -> Option<NonNull<Node<T>>> {
        return self.next;
    }

    pub fn add_next(&mut self, value: T) {
        let next_node = Node {
            value,
            next: None,
        };
        let leaked = Box::leak(Box::new(next_node));
        let non_null = NonNull::from_ref(&*leaked);
        self.next = Some(non_null);
    }
}

pub struct LinkedList<T: PartialEq> {
    head: Option<NonNull<Node<T>>>,
    len: usize
}

//  PatialEq pode ser qualquer tipo de dado
//  que possa ser comparado com "=="
impl<T: PartialEq> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, value: T) {
        
        if self.len == 0 {
            let node = Node::new(value);
            let leaked = Box::leak(Box::new(node));
            let non_null_node = NonNull::from_ref(&*leaked);

            self.head = Some(non_null_node);
            self.len += 1;
            return;
        };

        let current_node = self.head.unwrap();  //  garantido que existe, visto que len > 0
        let mut current_node: &mut Node<T> = unsafe {&mut *current_node.as_ptr()};  

        while let Some(next_node) = current_node.next() {
            current_node = unsafe {&mut *next_node.as_ptr()};
        }

        current_node.add_next(value);
        self.len += 1;
    }

    pub fn remove(&mut self, value: T) {
        if self.len == 0 {return;}

        let current_node = unsafe {self.head.unwrap().as_ref()};  //  garantido que existe, visto que len > 0
        if current_node.value == value {
            self.head = None;
            self.len -= 1;
            drop(Box::new(current_node));   //  desaloca current node
            return
        }

        let mut past_node = unsafe {self.head.unwrap().as_mut()};
        let mut current_node = unsafe { 
            let Some(n) = current_node.next else {return;};
            n.as_ref()
        };

        while current_node.value != value {
            if current_node.next.is_none() {return;}

            past_node = current_node;
            current_node = unsafe { past_node.next.unwrap().as_ref() };
        }

        //  nesse ponto o valor está no current_node 
        if let Some(next_node) = current_node.next {
            past_node.next = Some(next_node);
        } 
        else {
            past_node.next = None;
        }
        self.len -= 1;
        drop(Box::new(current_node));   //  desaloca current node
    }


    pub fn list(&self) -> Vec<&T> {
        dbg!("len: {}", self.len);
        let mut vec: Vec<&T> = Vec::with_capacity(self.len);

        let Some(current_node) = self.head.as_ref() else {return vec;};
        let mut current_node: &Node<T> = unsafe {current_node.as_ref()}; 

        vec.push(&current_node.value);

        for _ in 1..self.len {
            current_node = unsafe { current_node.next().unwrap().as_ref() };
            vec.push(&current_node.value);
        }

        return vec;
    }


}
