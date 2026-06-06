mod avl;
mod node;

use std::time::Instant;

use crate::avl::AVLTree;

fn main() {
    let mut tree = AVLTree::new();

    // Inserir 2 milhões de usuários
    println!("===============================================");
    println!("Inserindo 2 milhões de usuários");
    let start_insert = Instant::now();
    for id in 1..=2_000_000 {
        // ID 1 -> Score 1.999.999
        // ID 2.000.000 -> Score 0
        let score = 2_000_000 - id;
        tree.insert(id, score);
    }
    println!("   Concluído em: {:?}", start_insert.elapsed());

    // Remover os 200 mil piores jogadores
    println!("===============================================");
    println!("Removendo os 200 mil piores jogadores");
    let start_remove = Instant::now();
    for score in 0..200_000 {
        tree.remove(&score);
    }
    println!("   Concluído em: {:?}", start_remove.elapsed());

    // Consultas RANK x
    println!("===============================================");
    println!("Realizando 100 mil consultas RANK");
    let start_rank = Instant::now();
    for x in 900_000..=999_999 {
        let _ = tree.rank(&x);
    }
    println!("   Tempo gasto (RANK): {:?}", start_rank.elapsed());

    // Consultas KTH k
    println!("===============================================");
    println!("Realizando 100 mil consultas KTH");
    let start_kth = Instant::now();
    for k in 1..=100_000 {
        let _ = tree.kth(k);
    }
    println!("   Tempo gasto (KTH): {:?}", start_kth.elapsed());
}
