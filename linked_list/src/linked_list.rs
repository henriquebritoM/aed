
pub struct Node<T: PartialEq> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: PartialEq> Node<T> {
    pub fn new(value: T) -> Node<T> {
        return Node {
            value,
            next: None,
        };
    }

    pub fn next(&mut self) -> Option<&mut Box<Node<T>>> {
        return match self.next.as_mut() {
            Some(n) => Some(n),
            None => None,
        };
    }

    pub fn add_next(&mut self, value: T) {
        let next_node = Node {
            value,
            next: None,
        };
        self.next = Some(Box::new(next_node));
    }
}

pub struct LinkedList<T: PartialEq> {
    head: Option<Node<T>>,
    len: usize
}

impl<T: PartialEq> LinkedList<T> {

    pub fn new() -> LinkedList<T> {
        return LinkedList {
            head: None,
            len: 0,
        };
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn push(&mut self, value: T) {

        self.len += 1;

        let Some(mut current_node) = self.head.as_mut() else {
            self.head = Some(Node::new(value));
            return;
        };

        while current_node.next.is_some() {
            current_node = current_node.next().unwrap();
        }
        current_node.add_next(value);
    }

    pub fn remove(&mut self, value: T) {

        let Some(head) = self.head.as_mut() else {return;};

        if head.value == value {
            self.len -= 1;
            //  take torna head.next = None
            //  isso não importa, pois head vai ser descartado e 
            //  seu conteúdo anexado a self.head
            let Some(next_node) = head.next.take() else {
                self.head = None;
                return;
            };
            // restaura o valor de self.head
            self.head = Some(*next_node);
            return;
        }

        let mut current_node: &mut Node<T> = head;

        //  exit quando o próximo node tiver o elemento desejado
        //  ou quando chegar ao fim da lista
        while let Some(next_node) = current_node.next() {

             //  value está no current_node.next()
            if next_node.value == value {break;}
            current_node = current_node.next().unwrap();    //  current_node.next() não contem o valor
        }

        //  loop anterior chegou ao fim da lista e não enconrou o valor, retorna
        let Some(next_node) = current_node.next() else {return;};

        current_node.next = match next_node.next.take() {
            Some(node) => Some(node),
            None => None,
        };

        self.len -= 1;

    }

    pub fn list(&self) -> Vec<&T> {
        dbg!("len: {}", self.len);
        let mut vec: Vec<&T> = Vec::with_capacity(self.len);

        let Some(mut current_node) = self.head.as_ref() else {return vec;};
        vec.push(&current_node.value);

        while let Some(next) = &current_node.next {
            vec.push(&next.value);
            current_node = &next;
        }

        return vec;
    }

}