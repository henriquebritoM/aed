use std::fmt::{Debug, Display};

use bst::BST;


struct Node {
    id: i32,
    nome: String,
    idade: i32
}

impl Node {
    pub fn new(id: i32, nome: &str, idade: i32) -> Node {
        return Node { id, nome: nome.to_string(), idade };
    }

    pub fn with_id(id: i32) -> Node {
        Node { id, nome: "".to_string(), idade: 0}
    }
}

//  Definindo o critétio de como eu quero que os Nodes sejam comparados
//  Neste caso, queremos comparar apenas os IDs
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

//  Definindo o critério de ordenação dos Nodes
//  Neste caso, também queremos comparar apenas os IDs
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.id.partial_cmp(&other.id)
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {} Nome: {} Idade: {}", self.id, self.nome, self.idade)
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}", self.id)
    }
}

fn main() {

    //  1) inicie uma bst vazia
    let mut bst: BST<Node> = BST::new();

    //  2) Insira os registros, em ordem
    {
        bst.insert(Node::new(16, "Alex", 18));
        bst.insert(Node::new(8, "Ana", 15));
        bst.insert(Node::new(24, "Bruno", 21));
        bst.insert(Node::new(4, "Maria", 17));
        bst.insert(Node::new(12, "Lucas", 28));
        bst.insert(Node::new(20, "Isabela", 18));
        bst.insert(Node::new(28, "Rafael", 14));
        bst.insert(Node::new(2, "Laura", 25));
        bst.insert(Node::new(6, "Pedro", 18));
        bst.insert(Node::new(10, "Sofia", 17));
        bst.insert(Node::new(14, "Gabriel", 19));
        bst.insert(Node::new(18, "Helena", 20));
        bst.insert(Node::new(22, "Arthur", 18));
        bst.insert(Node::new(26, "Beatriz", 16));
        bst.insert(Node::new(30, "Mateus", 19));
        bst.insert(Node::new(5, "Alice", 17));
        bst.insert(Node::new(17, "Davi", 18));
        bst.insert(Node::new(19, "Livia", 21));
    }

    //  3) Vizualizar em ordem crescente e decrescente
    println!("Btree em ordem crescente");
    println!("{:#?}\n", bst.walk());

    println!("Btree em ordem decrescente");
    println!("{:#?}\n", bst.walk_reverse());

    //  4) Vizualizar a árvore
    println!("Vizualização em árvore:");
    bst.show_tree();

    //  5) Vizualizar relatório estatístico
    relatorio_estatistico(&bst);

    //  6) Remoção dos nós: 8, 24, 4, 30
    println!("Removendo os IDs 8, 24, 4, 30");
    bst.delete(&Node::with_id(8));
    bst.delete(&Node::with_id(24));
    bst.delete(&Node::with_id(5));
    bst.delete(&Node::with_id(30));

    //  7) Vizualizar novos relatórios estatísticos
    relatorio_estatistico(&bst);

    //  8) Vizualizar a árvore final
    println!("Vizualização em árvore:");
    bst.show_tree();
}


fn relatorio_estatistico<T: Display + PartialOrd> (btree: &BST<T>) {
    println!("\n-----------------------------\nRelatório:");
    println!("È uma bst? {}", btree.is_bst());
    println!("Esta balanceada? {}", btree.is_balanced());
    println!("Altura: {}", btree.altura());
    println!("Número de node: {}", btree.total_len());
    println!("Número de folhas: {}", btree.leafs_len());
    println!("\n");
}