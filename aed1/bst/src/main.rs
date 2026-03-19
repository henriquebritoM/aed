use std::fmt::{Debug, Display};

use bst::BST;


struct Registro {
    id: i32,
    nome: String,
    idade: i32
}

impl Registro {
    pub fn new(id: i32, nome: &str, idade: i32) -> Registro {
        return Registro { id, nome: nome.to_string(), idade };
    }

    pub fn with_id(id: i32) -> Registro {
        Registro { id, nome: "".to_string(), idade: 0}
    }
}

//  Definindo o critétio de como eu quero que os Registros sejam comparados
//  Neste caso, queremos comparar apenas os IDs
impl PartialEq for Registro {
    fn eq(&self, other: &Registro) -> bool {
        self.id == other.id
    }
}

//  Definindo o critério de ordenação dos Registros
//  Neste caso, também queremos comparar apenas os IDs
impl PartialOrd for Registro {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.id.partial_cmp(&other.id)
    }
}

impl Debug for Registro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {} Nome: {} Idade: {}", self.id, self.nome, self.idade)
    }
}
impl Display for Registro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}", self.id)
    }
}

fn main() {

    //  1) inicie uma bst vazia
    let mut bst: BST<Registro> = BST::new();

    //  2) Insira os registros, em ordem
    {
        bst.insert(Registro::new(16, "Alex", 18));
        bst.insert(Registro::new(8, "Ana", 15));
        bst.insert(Registro::new(24, "Bruno", 21));
        bst.insert(Registro::new(4, "Maria", 17));
        bst.insert(Registro::new(12, "Lucas", 28));
        bst.insert(Registro::new(20, "Isabela", 18));
        bst.insert(Registro::new(28, "Rafael", 14));
        bst.insert(Registro::new(2, "Laura", 25));
        bst.insert(Registro::new(6, "Pedro", 18));
        bst.insert(Registro::new(10, "Sofia", 17));
        bst.insert(Registro::new(14, "Gabriel", 19));
        bst.insert(Registro::new(18, "Helena", 20));
        bst.insert(Registro::new(22, "Arthur", 18));
        bst.insert(Registro::new(26, "Beatriz", 16));
        bst.insert(Registro::new(30, "Mateus", 19));
        bst.insert(Registro::new(5, "Alice", 17));
        bst.insert(Registro::new(17, "Davi", 18));
        bst.insert(Registro::new(19, "Livia", 21));
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
    bst.delete(&Registro::with_id(8));
    bst.delete(&Registro::with_id(24));
    bst.delete(&Registro::with_id(5));
    bst.delete(&Registro::with_id(30));

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
    println!("Número de Registro: {}", btree.total_len());
    println!("Número de folhas: {}", btree.leafs_len());
    println!("\n");
}