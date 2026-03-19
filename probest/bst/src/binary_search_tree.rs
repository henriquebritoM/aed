/* 
 *  DISCLAIMER
 *  Por padrão, em rust é usado o Smart Pointer Box<T>, 
 *  que é um unique pointer, ponteiros com referência contada
 *  também existem, mas acredito que não sejam adequados pois
 *  ocupam mais memória e a tarefa não exige uma complexidade
 *  desse nível.
 *  Com base nisso, usarei o NonNull<T>, que é um wrapper em 
 *  torno dos raw pointers (*const T e *mut T), com o 
 *  diferencial de não poder ser nulo, permitindo 
 *  algumas otimizações do compilador.
*/

use std::{cmp, fmt::Display, ptr::NonNull};

/* 
 *  PartialOrd é o trait (interface) atribuido a types que suportam 
 *  comparações ">", "<", "==", etc
 *  Option<NonNull<T>> tem o mesmo tamanho de um NonNull<T>,
 *  pois o compilador utiliza o ponteiro null como Option(None)
*/
pub struct Node<T: PartialOrd> {
    pub(crate) value: T,
    pub(crate) parent: Option<NonNull<Node<T>>>,
    pub(crate) left: Option<NonNull<Node<T>>>,
    pub(crate) right: Option<NonNull<Node<T>>>,
}

impl <T: PartialOrd> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            parent: None,
            left: None,
            right: None,
        }
    }

    /// Uma leaf é um node sem filhos
    pub fn is_leaf(&self) -> bool {
        return match (self.left, self.right) {
            (None, None) => true,
            _ => false
        }
    }
}

pub struct BST<T: PartialOrd> {
    root: Option<NonNull<Node<T>>>
}

//  Implementação da interface pública
impl <T: PartialOrd> BST<T> {

    /// Retorna uma nova instância de BST
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none() 
    }

    pub fn search(&self, value: &T) -> bool {
        match self.get(value) {
            Some(_) => true,
            None => false,
        }
    }

    ///  Adiciona um elemento à BST
    ///  **false** se o elemento já existia
    ///  **true** se o elemento foi adicionado <br>
    pub fn insert(&mut self, value: T) -> bool {

        let node = Node::new(value);
        let leaked = Box::leak(Box::new(node));
        let mut node_ptr = NonNull::from_mut(leaked);

        //  Inserir na root vazia
        if self.root.is_none() {
            self.root = Some(node_ptr);
            unsafe { node_ptr.as_mut().parent = None };
            return true;
        }

        return BST::insert_recursive(self.root.unwrap(), node_ptr);
    }

    /// Remove um elemento da BST <Br>
    /// **false** elemento não encontrado <br>
    /// **true** elemento encontrado e deletado
    pub fn delete(&mut self, value: &T) -> bool {

        let Some(node) = BST::get_node(self.root, value) else {
            return false; //    não há node para deletar
        };
        self.delete_node(node);
        
        return true;
    }
    
    pub fn get(&self, value: &T) -> Option<&T> {
        let node = BST::get_node(self.root, value);
        match node {
            Some(n) => unsafe { Some(&n.as_ref().value) },
            None => None,
        }
    }

    pub fn minimum(&self) -> Option<&T> {

        let minimum = BST::minimum_helper(self.root?);

        unsafe {
            return Some(&minimum.as_ref().value);
        };
    }

    pub fn maximum(&self) -> Option<&T> {
        let maximum = BST::maximum_helper(self.root?);

        unsafe {
            return Some(&maximum.as_ref().value);
        }
    }

    pub fn total_len(&self) -> usize {
        self.walk().len()

    }

    pub fn is_balanced(&self) -> bool {
        BST::is_balanced_recursive(self.root)
    }

    pub fn is_bst(&self) -> bool {
        if self.root.is_none() {return true;}
        BST::is_bst_recursive(self.root.unwrap())
    }

    pub fn leafs_len(&self) -> usize {
        let mut n: usize = 0;

        if self.root.is_none() {return 0;}

        BST::leafs_len_recursive(self.root.unwrap(), &mut n);
        n
    }
    
    pub fn clear(&mut self) {
        while let Some(node) = self.root {
            self.delete_node(node);
        }
    }
    
    pub fn altura(&self) -> usize {
        BST::altura_recursiva(self.root)
    }

    //  walk in order
    pub fn walk(&self) -> Vec<&T> {
        let mut vec: Vec<&T> = Vec::new();
        BST::walk_recursive(self.root, &mut vec);
        return vec;
    }

    pub fn walk_reverse(&self) -> Vec<&T> {
        let mut vec = Vec::new();
        BST::walk_reverse_recursive(self.root, &mut vec);
        vec
    }


}

//  Interface privada
impl<T: PartialOrd> BST<T> {

    /// Função helper para BST::insert()
    fn insert_recursive(mut node: NonNull<Node<T>>, mut target: NonNull<Node<T>>) -> bool {
        
        unsafe {
            let target_value = &target.as_ref().value;
            let node_value = &node.as_ref().value;

            if target_value == node_value { return false; }
            
            if target_value < node_value {
                if let Some(left) = node.as_ref().left {
                    return BST::insert_recursive(left, target);
                }
                target.as_mut().parent = Some(node);
                node.as_mut().left = Some(target);
            }

            else if target_value > node_value {
                if let Some(right) = node.as_ref().right {
                    return BST::insert_recursive(right, target);
                }
                target.as_mut().parent = Some(node);
                node.as_mut().right = Some(target);
            }

            return true;

        }
    }

    //  Retorna um ponteiro (não único) para um Node
    fn get_node(node: Option<NonNull<Node<T>>>, value: &T) -> Option<NonNull<Node<T>>> {

        //  Chegou ao final da BST e o valor não foi encontrado
        let Some(mut root) = node else {
            return node;
        };

        //  Unsafe pois estamos de-referenciando um ponteiro, não há perigo
        //  pois todos os ponteiros são validados na criação
        unsafe {
            //  valor menor do que o root, comparar a esquerda
            if  value < &root.as_ref().value {
                return BST::get_node(root.as_mut().left, value);
            }
            //  valor maior do que o root, comparar a direita
            else if value > &root.as_ref().value {
                return BST::get_node(root.as_mut().right, value);
            }
            //  valor e  ncontrado, retornar o node (root)
            else {
                return node;
            }
        }
            
    }

    //  Retorna um ponteiro (não único) para o menor node da sub-árvore
    //  esse node pode ser a própria root, se não houver filhos
    fn minimum_helper(mut node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
    
            while let Some(left) = node.as_ref().left {
                node = left;
            }
    
            return node;
        }
    }

    //  Retorna um ponteiro (não único) para o maior node da sub-árvore
    //  esse node pode ser a própria root, se não houver filhos
    fn maximum_helper(mut node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
    
            while let Some(right) = node.as_ref().right {
                node = right;
            }
    
            return node;
        }
    }

    //  Retorna um ponteiro (não exclusivo) para o sucessor de um Node
    //  na bst.
    //  Pode ser que não haja um sucessor (None)
    fn sucessor(node: NonNull<Node<T>>) -> Option<NonNull<Node<T>>> {   
        let sucessor: Option<NonNull<Node<T>>>;

        unsafe {

            if let Some(right) = node.as_ref().right {
                sucessor = Some(BST::minimum_helper(right));
            }
            else {
                let mut parent= node.as_ref().parent; 

                while let Some(p) = parent {
                    if Some(node) == p.as_ref().right {
                        parent = p.as_ref().parent;
                    }
                }
                sucessor = parent;
            }

        }
        return sucessor;

    }

    //  Coloca n2 (e sua subarvore) no lugar de n1 (e sua subarvore) na árvore
    //  n1 é removido (mas não deletado) da árvore
    fn transplant(&mut self, n1: NonNull<Node<T>>, n2: Option<NonNull<Node<T>>>) { 

        //  não há nada o que trocar, ambos são o mesmo node
        if Some(n1) == n2 {return;};
        unsafe {

            if n1.as_ref().parent.is_none() {
                self.root = n2;
            }
            else {
                let mut parent = n1.as_ref().parent.unwrap();
    
                if Some(n1) == parent.as_ref().left {
                    parent.as_mut().left = n2;
                }
                else {
                    parent.as_mut().right = n2; 
                }
            }

            //  O parent de n1 aponta para n2
            // let n1_parent_ptr = self.get_parent_ptr(n1);
            // *n1_parent_ptr = n2;

            //  Se houver um n2, seu campo parent aponta para o parent de n1
            if let Some(mut n2) = n2 {
                n2.as_mut().parent = n1.as_ref().parent;
            }
        }
    }

    fn delete_node(&mut self, mut node: NonNull<Node<T>>) {
        unsafe {

            let left = node.as_mut().left.take();
            let right = node.as_mut().right.take();
    
            match (left, right) {
                (None, None) => {
                    self.transplant(node, None);
                },
                (None, Some(right)) => {
                    self.transplant(node, Some(right));
                },
                (Some(left), None) => {
                    self.transplant(node, Some(left));
                },
                (Some(mut left), Some(mut right)) => {
                    //  Há um filho a direita, ou seja, é impossível o unwarp falhar.
                    let mut sucessor = BST::minimum_helper(right);

                    //  Se o sucessor não está diretamente a direita do node, são necessários 2 transplantes
                    if sucessor.as_ref().parent != Some(node) {
                        //  sucessor é substituido por seu nó a direita na árvore
                        //  sucessor agora está fora da árvore
                        self.transplant(sucessor, sucessor.as_ref().right);

                        //  sucessor.right é ligado (e vice-versa) ao filho direiro de node
                        right.as_mut().parent = Some(sucessor);
                        sucessor.as_mut().right = Some(right);
                    }
    
                    //  sucessor.left é ligado (e vice-versa) ao filho esquerdo do node
                    left.as_mut().parent = Some(sucessor);
                    sucessor.as_mut().left = Some(left);

                    //  node é trocado por sucessor
                    //  node agora está fora da árvore
                    self.transplant(node, Some(sucessor));
                }
            }
    
            //  desaloca o node deletado
            drop(Box::from_raw(node.as_ptr()));
        }
    }

    fn walk_recursive(node: Option<NonNull<Node<T>>>, vec: &mut Vec<&T>)  {
        unsafe {
            if let Some(node) = node {
                BST::walk_recursive(node.as_ref().left, vec);
                vec.push(&node.as_ref().value);
                BST::walk_recursive(node.as_ref().right, vec);
            }
        }
    }

    fn altura_recursiva(node: Option<NonNull<Node<T>>>) -> usize {
        
        let Some(node) = node else {return 0;};

        unsafe {

            let left_height = BST::altura_recursiva(node.as_ref().left);
            let right_height = BST::altura_recursiva(node.as_ref().right);
            
            return cmp::max(left_height, right_height) + 1;
        }
    }

    //  é mais eficiente fazer outra função do que usar reverse no resultado da padrão
    fn walk_reverse_recursive(node: Option<NonNull<Node<T>>>, vec: &mut Vec<&T>)  {
        unsafe {
            if let Some(node) = node {
                BST::walk_reverse_recursive(node.as_ref().right, vec);
                vec.push(&node.as_ref().value);
                BST::walk_reverse_recursive(node.as_ref().left, vec);
            }
        }
    }

    // IsHeightBalanced(tree)
    //         (IsHeightBalanced(tree.left) and
    //             IsHeightBalanced(tree.right) and
                // abs(Height(tree.left) - Height(tree.right)) <= 1)

    fn is_balanced_recursive(node: Option<NonNull<Node<T>>>) -> bool {
        unsafe {
            let Some(node) = node else {return true;};

            let left_ok = BST::is_balanced_recursive(node.as_ref().left);
            let right_ok = BST::is_balanced_recursive(node.as_ref().right);
            let height_balanced = 
                BST::altura_recursiva(node.as_ref().left) as i32 - 
                BST::altura_recursiva(node.as_ref().right) as i32 <= 1;

            return left_ok && right_ok && height_balanced;
        }
    }

    fn is_bst_recursive(node: NonNull<Node<T>>) -> bool {
        unsafe {
            let (mut ok_left,  mut ok_right) = (true, true);

            if let Some(left) = node.as_ref().left {
                if left.as_ref().value > node.as_ref().value {return false}
                ok_left = BST::is_bst_recursive(left);
            }

            if ok_left == false {return false}

            if let Some(right) = node.as_ref().right {
                if right.as_ref().value < node.as_ref().value {return false}
                ok_right = BST::is_bst_recursive(right);
            }

            return ok_right;
        }
    }

    fn leafs_len_recursive(node: NonNull<Node<T>>, n: &mut usize) {
        unsafe {

            if node.as_ref().is_leaf() {
                *n += 1;
            }

            if let Some(left) = node.as_ref().left {
                BST::leafs_len_recursive(left, n);
            }
            if let Some(right) = node.as_ref().right {
                BST::leafs_len_recursive(right, n);
            }
        }
    }

}

//  Print da árvore para Ts que implementam Display
//  (Ts que podem ser printados)
//  Feito com IA para se parecer com um gerenciador de arquivos
impl<T: Display + PartialOrd> BST<T> {
    /// Método público para iniciar a impressão visual da árvore.
    pub fn show_tree(&self) {
        if let Some(root_ptr) = self.root {
            unsafe {
                BST::print_recursive(root_ptr, 0, true);
            }
        } else {
            println!("(Árvore Vazia)");
        }
    }

    // Função auxiliar recursiva que faz o trabalho pesado
    // Usa 'prefix' para desenhar a linha vertical da indentação
    unsafe fn print_recursive(node_ptr: NonNull<Node<T>>, depth: usize, eh_o_ultimo: bool) {
        let node = node_ptr.as_ref();

        // 1. Imprimir o nó atual
        // Cria a string de indentação e o prefixo (branch)
        let indent = "│   ".repeat(depth);
        let branch = if eh_o_ultimo { "└── " } else { "├── " };
        
        println!("{}{}{}", indent, branch, node.value);

        // O novo prefixo para os filhos é a indentação + o espaço vertical
        let new_indent = indent + if eh_o_ultimo { "    " } else { "│   " };
        
        // Determinar se o filho esquerdo será o "último" (se o direito for None)
        let left_is_last = node.right.is_none();

        // 2. Chamada Recursiva para o filho ESQUERDO
        if let Some(left_ptr) = node.left {
            // Passa a nova profundidade e se este filho será o último do seu nível
            Self::print_recursive(left_ptr, depth + 1, left_is_last);
        } else if node.right.is_some() {
            // Desenha um nó 'vazio' se o filho direito existir, para manter o alinhamento
            println!("{}{}└── (Vazio)", new_indent, if left_is_last { "    " } else { "│   " });
        }


        // 3. Chamada Recursiva para o filho DIREITO
        if let Some(right_ptr) = node.right {
            // O filho direito SEMPRE é o último elemento a ser desenhado naquele nível
            Self::print_recursive(right_ptr, depth + 1, true);
        }
    }
}
