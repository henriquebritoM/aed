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

use std::{iter::Successors, ptr::NonNull};

enum pos {
    Left,
    Right,
}

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

impl <T: PartialEq + PartialOrd> Node<T> {
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
impl <T: PartialEq + PartialOrd> BST<T> {

    /// Retorna uma nova instância de BST
    pub fn new() -> BST<T> {
        BST { root: Some(NonNull::dangling()), }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none() 
    }

    pub fn exists(&self, value: &T) -> bool {
        BST::find_recursive(&self.root, value)
    }

    ///  Adiciona um elemento à BST
    ///  **false** se o elemento já existia
    ///  **true** se o elemento foi adicionado <br>
    pub fn insert(&mut self, value: T) -> bool {

        let node = Node::new(value);
        let leaked = Box::leak(Box::new(node));
        let node_ptr = NonNull::from_mut(leaked);

        return BST::insert_recursive(&mut self.root, node_ptr);
    }

    /// Remove um elemento da BST <Br>
    /// **false** elemento não encontrado <br>
    /// **true** elemento encontrado e deletado
    pub fn delete(&mut self, value: &T) -> bool {
        
        //  essa variável é uma referência para o ponteiro 
        //  do parent
        //  mudando ela diretamente não precisamos trazer o 
        //  parent para o escopo
        let node_opt = BST::get_node(&mut self.root, value);
        let Some(mut node) = node_opt.take() else {
            return false;
        };

        unsafe {
            
            match (node.as_mut().left, node.as_mut().right) {
                (None, None) => {
                    *node_opt = None;
                },
                (None, Some(_)) => {
                    *node_opt = node.as_mut().right.take();
                },
                (Some(_), None) => {
                    *node_opt = node.as_mut().left.take();
                },
                (Some(_), Some(_)) => {
                    let sucessor = BST::sucessor(node);
                    self.transplant(node, n2);
                },
            }

            drop(Box::from_raw(node.as_ptr())); //  desaloca a memória do node
        }

        return true;
    }

    //  walk in order
    pub fn walk(&self) -> Vec<&Node<T>> {
        let mut vec: Vec<&Node<T>> = Vec::new();
        BST::walk_recursive(self.root, &mut vec);
        return vec;
    }


}

//  Interface privada
impl<T: PartialOrd> BST<T> {

    /// Função helper para BST::insert()
    fn insert_recursive(node: &mut Option<NonNull<Node<T>>>, target: NonNull<Node<T>>) -> bool {
        
        //  root == None -> Chegou a posição correta para adicionar o target 
        let Some(root) = node else {
            node.replace(target);   //  Troca o None por Some(target)
            return true;
        };

        //  Unsafe pois estamos de-referenciando um ponteiro, não há perigo
        //  pois todos os ponteiros são validados na criação
        unsafe {
            //  valor menor do que o root, adicionar a esqueda
            if  target.as_ref().value < root.as_ref().value {
                return BST::insert_recursive(&mut root.as_mut().left, target);
            }
            //  valor maior do que o root, adicionar a direita
            else if target.as_ref().value > root.as_ref().value {
                return BST::insert_recursive(&mut root.as_mut().right, target);
            }
            //  valor igual ao root, não adicionar nada
            else {
                return false;
            }
        }
    }

    fn find_recursive(node: &Option<NonNull<Node<T>>>, value: &T) -> bool {
        
        //  Chegou ao final da BST e o valor não foi encontrado
        let Some(root) = node else {
            return false;
        };

        //  Unsafe pois estamos de-referenciando um ponteiro, não há perigo
        //  pois todos os ponteiros são validados na criação
        unsafe {
            //  valor menor do que o root, comparar a esquerda
            if  value < &root.as_ref().value {
                return BST::find_recursive(&root.as_ref().left, value);
            }
            //  valor maior do que o root, comparar a direita
            else if value > &root.as_ref().value {
                return BST::find_recursive(&root.as_ref().right, value);
            }
            //  valor encontrado
            else {
                return true;
            }
        }
    }

    fn get_node<'a>(node: &'a mut Option<NonNull<Node<T>>>, value: &T) -> &'a mut Option< NonNull<Node<T>>> {

        //  Chegou ao final da BST e o valor não foi encontrado
        let Some(root) = node else {
            return node;
        };

        //  Unsafe pois estamos de-referenciando um ponteiro, não há perigo
        //  pois todos os ponteiros são validados na criação
        unsafe {
            //  valor menor do que o root, comparar a esquerda
            if  value < &root.as_ref().value {
                return BST::get_node(&mut root.as_mut().left, value);
            }
            //  valor maior do que o root, comparar a direita
            else if value > &root.as_ref().value {
                return BST::get_node(&mut root.as_mut().right, value);
            }
            //  valor e  ncontrado, retornar o node (root)
            else {
                return node;
            }
        }
            
    }

    unsafe fn minimun(node: NonNull<Node<T>>) -> NonNull<Node<T>> {

        let mut left = node.to_owned(); //  se não houver nós filhos é 

        while let Some(l) = left.as_ref().left {
            left = l;
        }

        return left;
    }

    fn sucessor(node: NonNull<Node<T>>) -> Option<NonNull<Node<T>>> {
        let sucessor: Option<NonNull<Node<T>>>;

        unsafe {

            if let Some(right) = node.as_ref().right {
                sucessor = Some(BST::minimun(right));
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

    //  Neste caso, é mais fácil passar os ponteiros por cópia e modificar os Nodes
    //  um a um
    fn transplant(&mut self, mut n1: NonNull<Node<T>>, mut n2: NonNull<Node<T>>) { 

        //  não há nada o que trocar, ambos são o mesmo node
        if n1 == n2 {return;};
        unsafe {
            //  impossível unwrap falhar, há dois nodes DIFERENTES, o que implica que há um root
            let n1_parent = n1.as_ref().parent;   
            let n2_parent = n2.as_ref().parent;
            let (n1_left, n1_right) = (n1.as_ref().left, n1.as_ref().right);
            let (n2_left, n2_right) = (n2.as_ref().left, n2.as_ref().right);
    
            //  Primeiramente invertemos os nós filhos dos dois
            n1.as_mut().left = n2_left;
            n1.as_mut().right = n2_right;
    
            n2.as_mut().left = n1_left;
            n2.as_mut().right = n1_right;
    
            if let Some(mut n1_parent) = n1_parent {
                n1_parent.as_mut().right = Some(n2);
    
            }
            else {
                self.root = Some(n2);
            }
    
            if let Some(mut n2_parent) = n2_parent {
                n2_parent.as_mut().right = Some(n1);
            }
            else {
                self.root = Some(n1);
            }
        }
    }

    fn walk_recursive(node: Option<NonNull<Node<T>>>, vec: &mut Vec<&Node<T>>)  {
        unsafe {
            if let Some(node) = node {
                BST::walk_recursive(node.as_ref().left, vec);
                vec.push(node.as_ref());
                BST::walk_recursive(node.as_ref().right, vec);
            }
        }
    }


}
