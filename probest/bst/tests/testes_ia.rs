//  ------- DISCLAIMER -------
//  Os testes abaixo foram feitos com ia como um superset
//  dos testes básicos que eu havia montado

#[cfg(test)]
mod tests {
    use bst::BST; // Ajuste conforme o nome do seu módulo

    // =================================================================
    //  SETUP BUILDERS (Funções Auxiliares)
    // =================================================================

    /// Cria uma árvore desbalanceada à direita (Skewed Right).
    /// Estrutura: 1 -> 2 -> 3 -> 4 -> 5
    /// In-order: [1, 2, 3, 4, 5]
    fn setup_skewed_tree() -> BST<i32> {
        let mut bst = BST::new();
        for i in 1..=5 {
            bst.insert(i);
        }
        bst
    }

    /// Cria uma árvore balanceada completa de 3 níveis (7 nós).
    /// Raiz: 3. Filhos: 1, 5. Netos: 0, 2, 4, 6.
    /// In-order: [0, 1, 2, 3, 4, 5, 6]
    fn setup_complete_tree() -> BST<i32> {
        let mut bst = BST::new();
        // Ordem de inserção para garantir balanceamento:
        let insertion_order = [3, 1, 5, 0, 2, 4, 6];
        for &val in &insertion_order {
            bst.insert(val);
        }
        bst
    }

    /// Cria uma árvore pequena balanceada (3 nós).
    /// Raiz: 50. Filhos: 30, 70.
    fn setup_simple_tree() -> BST<i32> {
        let mut bst = BST::new();
        bst.insert(50);
        bst.insert(30);
        bst.insert(70);
        bst
    }

    // =================================================================
    //  GRUPO 1: TESTES DE INSERÇÃO E CAMINHAMENTO (WALK)
    // =================================================================

    #[test]
    fn insert_single_node() {
        let mut bst = BST::new();
        assert!(bst.insert(1));
        assert_eq!(vec![&1], bst.walk());
    }

    #[test]
    fn insert_duplicates_should_fail() {
        let mut bst = setup_simple_tree(); // Já tem 30, 50, 70
        assert_eq!(bst.insert(50), false, "Não deve inserir duplicatas");
        assert_eq!(bst.total_len(), 3, "Tamanho não deve mudar");
    }

    #[test]
    fn walk_skewed_tree_preserves_order() {
        let bst = setup_skewed_tree();
        assert_eq!(vec![&1, &2, &3, &4, &5], bst.walk());
    }

    #[test]
    fn walk_complete_tree_preserves_order() {
        let bst = setup_complete_tree();
        assert_eq!(vec![&0, &1, &2, &3, &4, &5, &6], bst.walk());
    }

     #[test]
    fn walk_reverse_complete_tree_preserves_order() {
        let bst = setup_complete_tree();
        assert_eq!(vec![&6, &5, &4, &3, &2, &1, &0], bst.walk_reverse());
    }   

    // =================================================================
    //  GRUPO 2: TESTES DE BUSCA (SEARCH)
    // =================================================================

    #[test]
    fn search_existing_elements() {
        let bst = setup_complete_tree();
        // Testa raiz, folhas e nós internos
        assert!(bst.search(&3)); 
        assert!(bst.search(&0)); 
        assert!(bst.search(&6));
    }

    #[test]
    fn search_non_existent_elements() {
        let bst = setup_complete_tree(); // Valores de 0 a 6
        assert_eq!(bst.search(&-1), false);
        assert_eq!(bst.search(&10), false);
        assert_eq!(bst.search(&7), false);
    }

    // =================================================================
    //  GRUPO 3: TESTES DE REMOÇÃO (DELETE)
    // =================================================================

    #[test]
    fn delete_leaf_node() {
        // Árvore: 1 -> 2 -> 3 -> 4 -> 5
        let mut bst = setup_skewed_tree();
        
        bst.delete(&5); // Remove a última folha

        assert_eq!(vec![&1, &2, &3, &4], bst.walk());
    }

    #[test]
    fn delete_multiple_leaf_nodes() {
        // Árvore completa. Folhas são: 0, 2, 4, 6
        let mut bst = setup_complete_tree();

        bst.delete(&0);
        bst.delete(&2);
        bst.delete(&4);
        bst.delete(&6);

        // Restam os nós internos: 1, 3, 5
        assert_eq!(vec![&1, &3, &5], bst.walk());
    }

    #[test]
    fn delete_internal_node_one_child() {
        // Árvore: 1 -> 2 -> 3 -> 4 -> 5
        let mut bst = setup_skewed_tree();

        bst.delete(&2); // 2 tem filho 3

        assert_eq!(vec![&1, &3, &4, &5], bst.walk());
    }

    #[test]
    fn delete_internal_node_two_children() {
        // Árvore completa. Nó 1 tem filhos 0 e 2. Nó 5 tem filhos 4 e 6.
        let mut bst = setup_complete_tree();

        bst.delete(&1);
        bst.delete(&5);

        assert_eq!(vec![&0, &2, &3, &4, &6], bst.walk());
    }

    #[test]
    fn delete_root_with_one_child() {
        // Árvore: 1 -> 2 -> 3 -> 4 -> 5 (Raiz é 1, filho direito é 2)
        let mut bst = setup_skewed_tree();

        bst.delete(&1);

        assert_eq!(vec![&2, &3, &4, &5], bst.walk());
    }

    #[test]
    fn delete_root_with_two_children() {
        // Árvore completa. Raiz é 3, filhos 1 e 5.
        let mut bst = setup_complete_tree();

        bst.delete(&3);

        // A nova raiz será escolhida (geralmente sucessor ou antecessor),
        // mas o caminhamento deve permanecer ordenado.
        assert_eq!(vec![&0, &1, &2, &4, &5, &6], bst.walk());
    }

    #[test]
    fn delete_all_nodes_in_order() {
        let mut bst = setup_complete_tree();
        let deletion_order = [0, 1, 2, 3, 4, 5, 6]; // Ordem sequencial

        for val in deletion_order {
            bst.delete(&val);
        }

        assert!(bst.walk().is_empty());
        assert!(bst.is_empty());
    }

    #[test]
    fn delete_all_nodes_random_order() {
        let mut bst = setup_complete_tree();
        // Remove raiz, folhas, nós internos misturados
        let deletion_order = [3, 0, 6, 1, 5, 2, 4]; 

        for val in deletion_order {
            bst.delete(&val);
        }

        assert!(bst.walk().is_empty());
    }

    #[test]
    fn test_delete_empty_tree() {
        let mut bst: BST<i32> = BST::new();
        // Tenta deletar um elemento que não existe, não deve travar
        bst.delete(&10);
        assert!(bst.is_empty());
    }

    #[test]
    fn test_delete_only_root_node() {
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.delete(&10);
        assert!(bst.is_empty());
    }

    #[test]
    fn test_delete_leaf_from_complete_tree() {
        // Árvore: [0, 1, 2, 3, 4, 5, 6]. 0 é uma folha.
        let mut bst = setup_complete_tree();
        bst.delete(&0);
        assert_eq!(bst.total_len(), 6);
        assert_eq!(bst.walk(), vec![&1, &2, &3, &4, &5, &6]);
        assert_eq!(bst.search(&0), false);
    }

    #[test]
    fn test_delete_leaf_from_skewed_tree() {
        // Árvore: 1 -> 2 -> 3 -> 4 -> 5. 5 é a folha.
        let mut bst = setup_skewed_tree();
        bst.delete(&5);
        assert_eq!(bst.total_len(), 4);
        assert_eq!(bst.walk(), vec![&1, &2, &3, &4]);
    }

    #[test]
    fn test_delete_node_one_right_child() {
        // Cenário: 10 (root), 20 (R), 30 (RR). Deleta 20.
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(20);
        bst.insert(30); // 20 tem apenas 30 como filho

        bst.delete(&20);
        assert_eq!(bst.walk(), vec![&10, &30]);
        assert_eq!(bst.total_len(), 2);
    }

    #[test]
    fn test_delete_node_one_left_child() {
        // Cenário: 30 (root), 20 (L), 10 (LL). Deleta 20.
        let mut bst: BST<i32> = BST::new();
        bst.insert(30);
        bst.insert(20);
        bst.insert(10); // 20 tem apenas 10 como filho

        bst.delete(&20);
        assert_eq!(bst.walk(), vec![&10, &30]);
        assert_eq!(bst.total_len(), 2);
    }

    #[test]
    fn test_delete_root_one_child() {
        // Árvore Skewed: 1 (R), 2 (RR), 3 (RRR). Deleta a raiz 1.
        let mut bst = setup_skewed_tree(); // 1 -> 2 -> 3 -> 4 -> 5
        bst.delete(&1);
        assert_eq!(bst.walk(), vec![&2, &3, &4, &5]);
        assert_eq!(bst.total_len(), 4);
    }

    #[test]
    fn test_delete_root_two_children_successor_not_immediate() {
        // Árvore: 3 (root), 1 (L), 5 (R), 4 (RL), 6 (RR). Sucessor de 3 é 4.
        let mut bst = setup_complete_tree(); // [0, 1, 2, 3, 4, 5, 6]
        
        // A remoção de 3 deve fazer o sucessor (4) subir.
        // O 4 deve adotar os filhos do 5 (se houvesse) e o 5 deve ser adotado pelo 4.
        bst.delete(&3);

        assert_eq!(bst.total_len(), 6);
        // Verificar que 4 subiu e a ordem está correta
        assert_eq!(bst.walk(), vec![&0, &1, &2, &4, &5, &6]);
        assert_eq!(bst.search(&3), false);
    }

    #[test]
    fn test_delete_root_two_children_successor_is_immediate() {
        // Cenário onde o sucessor é o filho direito imediato (o caso mais simples).
        // Tree: 5 (R), 3 (L), 6 (RR) -> Sucessor de 5 é 6.
        let mut bst: BST<i32> = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(6); 
        
        // Deleta 5. O 6 deve subir, mantendo o 3 na esquerda.
        bst.delete(&5);

        assert_eq!(bst.total_len(), 2);
        assert_eq!(bst.walk(), vec![&3, &6]);
        assert_eq!(bst.search(&5), false);
    }

    #[test]
    fn test_delete_internal_node_two_children() {
        // Árvore: [0, 1, 2, 3, 4, 5, 6]. Deleta 1. Sucessor de 1 é 2.
        let mut bst = setup_complete_tree(); // Raiz é 3
        
        bst.delete(&1); // 1 tem filhos 0 e 2. O sucessor é 2.

        assert_eq!(bst.total_len(), 6);
        // 2 sobe para a posição de 1, mantendo 0 na sua esquerda.
        assert_eq!(bst.walk(), vec![&0, &2, &3, &4, &5, &6]);
        assert_eq!(bst.search(&1), false);
    }

    #[test]
    fn test_delete_complex_two_children() {
        // Cenário: 50 (R), 30 (L), 70 (R), 60 (RL), 80 (RR). Deleta 70. Sucessor de 70 é 80.
        let mut bst: BST<i32> = BST::new();
        let values = [50, 30, 70, 60, 80];
        for &v in &values { bst.insert(v); }
        
        // Deleta 70. Sucessor 80 sobe. 80 não tem filhos.
        bst.delete(&70);

        assert_eq!(bst.total_len(), 4);
        assert_eq!(bst.walk(), vec![&30, &50, &60, &80]);
        assert_eq!(bst.search(&70), false);
    }

    #[test]
    fn test_delete_non_existent() {
        let mut bst = setup_complete_tree();
        let original_len = bst.total_len();
        
        // Não deve deletar nada, o tamanho deve permanecer o mesmo
        bst.delete(&99); 
        
        assert_eq!(bst.total_len(), original_len);
        assert_eq!(bst.walk(), vec![&0, &1, &2, &3, &4, &5, &6]);
    }

    #[test]
    fn test_delete_all_nodes_random_order() {
        let mut bst = setup_complete_tree();
        // Ordem: Root(3), Leaves(0, 6), Internal(1), TwoChildren(5, 2), LastNode(4)
        let deletion_order = [3, 0, 6, 1, 5, 2, 4]; 

        for val in deletion_order {
            bst.delete(&val);
        }
        assert!(bst.is_empty());
        assert_eq!(bst.total_len(), 0);
    }

    #[test]
    fn test_delete_tree_restructure_check() {
        // Árvore: 3 (R), 1 (L), 5 (R), 4 (RL), 6 (RR).
        let mut bst = setup_complete_tree();
        
        // Deleta um nó interno de 2 filhos (5)
        bst.delete(&5); // Sucessor de 5 é 6. 6 sobe para a posição de 5.
        assert_eq!(bst.walk(), vec![&0, &1, &2, &3, &4, &6]);
        
        // Deleta o nó 6 (que agora tem 4 como filho esquerdo). 6 tem 1 filho (4).
        bst.delete(&6); // 4 sobe para a posição de 6.
        assert_eq!(bst.walk(), vec![&0, &1, &2, &3, &4]);
        
        assert_eq!(bst.total_len(), 5);
    }

    // =================================================================
    //  GRUPO 4: TESTES DE MÉTRICAS (LEN, LEAFS, PROPERTIES)
    // =================================================================

    #[test]
    fn count_total_nodes() {
        assert_eq!(BST::<i32>::new().total_len(), 0);
        assert_eq!(setup_simple_tree().total_len(), 3);
        assert_eq!(setup_skewed_tree().total_len(), 5);
        assert_eq!(setup_complete_tree().total_len(), 7);
    }

    #[test]
    fn count_leaf_nodes() {
        assert_eq!(BST::<i32>::new().leafs_len(), 0);
        
        // Simples: 30 e 70 são folhas
        assert_eq!(setup_simple_tree().leafs_len(), 2);
        
        // Skewed: Apenas o 5 é folha
        assert_eq!(setup_skewed_tree().leafs_len(), 1);
        
        // Completa: 0, 2, 4, 6 são folhas
        assert_eq!(setup_complete_tree().leafs_len(), 4);
    }

    #[test]
    fn check_bst_property() {
        let bst = setup_complete_tree();
        // Assume que você tem um método is_bst() implementado
        // ou verifica se o walk está estritamente crescente
        assert!(bst.is_bst()); 
        
        let walk = bst.walk();
        for i in 0..walk.len()-1 {
            assert!(walk[i] < walk[i+1]);
        }
    }

    #[test]
    fn clear_tree_resets_state() {
        let mut bst = setup_complete_tree();
        bst.clear();
        assert!(bst.is_empty());
        assert_eq!(bst.total_len(), 0);
    }

    #[test]
    fn test_altura_empty() {
        // Altura de uma árvore vazia é 0
        let bst: BST<i32> = BST::new();
        assert_eq!(bst.altura(), 0);
    }

    #[test]
    fn test_altura_single_node() {
        // Altura de uma árvore com apenas a raiz é 1
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        assert_eq!(bst.altura(), 1);
    }

    #[test]
    fn test_altura_skewed_right() {
        // Árvore: 1 -> 2 -> 3 -> 4 -> 5 (altura deve ser 5)
        let bst = setup_skewed_tree();
        assert_eq!(bst.altura(), 5);
    }

    #[test]
    fn test_altura_skewed_left() {
        // Árvore: 5 -> 4 -> 3 -> 2 -> 1 (altura deve ser 5)
        let mut bst: BST<i32> = BST::new();
        bst.insert(5);
        bst.insert(4);
        bst.insert(3);
        bst.insert(2);
        bst.insert(1);
        assert_eq!(bst.altura(), 5);
    }

    #[test]
    fn test_altura_balanced_3_nodes() {
        // Árvore: 50 (R), 30 (L), 70 (R). Altura = 2
        let bst = setup_simple_tree(); // Assumindo que setup_simple_tree cria 50, 30, 70
        assert_eq!(bst.altura(), 2);
    }

    #[test]
    fn test_altura_complete_7_nodes() {
        // Árvore: [0..6]. Total de 7 nós. log2(7) arredondado para cima é 3.
        let bst = setup_complete_tree(); // Assumindo que setup_complete_tree cria 7 nós
        assert_eq!(bst.altura(), 3);
    }

    #[test]
    fn test_altura_complex_15_nodes() {
        // Árvore completa de 4 níveis. 15 nós. log2(15) arredondado para cima é 4.
        let mut bst: BST<i32> = BST::new();
        // 1º Nível (8)
        // 2º Nível (4, 12)
        // 3º Nível (2, 6, 10, 14)
        // 4º Nível (1, 3, 5, 7, 9, 11, 13, 15)
        let values = vec![8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
        for &val in &values {
            bst.insert(val);
        }
        assert_eq!(bst.altura(), 4);
    }

    #[test]
    fn test_altura_asymmetric() {
        // Cenário: Caminho mais longo está na esquerda (Altura 4)
        // 10 -> 5 (L) -> 2 (LL) -> 1 (LLL)
        // 10 -> 15 (R)
        let mut bst: BST<i32> = BST::new();
        bst.insert(10);
        bst.insert(15);
        bst.insert(5);
        bst.insert(2);
        bst.insert(1);
        assert_eq!(bst.altura(), 4);
    }

    #[test]
    fn test_altura_after_deletion() {
        // Teste se a altura é recalculada corretamente após a deleção de um nó
        let mut bst = setup_complete_tree(); // Altura inicial: 3

        // Deleta uma folha (0). Altura deve permanecer 3.
        bst.delete(&0);
        assert_eq!(bst.altura(), 3, "Deletar folha não deve mudar a altura");

        // Deleta a raiz (3). O sucessor sobe e a árvore deve se reestruturar.
        // A altura pode cair para 3 ou 2, dependendo da reestruturação.
        // Vamos assumir que a árvore ainda tem 3 níveis:
        bst.delete(&3);
        assert_eq!(bst.altura(), 3, "Altura deve permanecer 3 ou cair para 2");

        // Deleta todos os nós na sub-árvore esquerda (1 e 2)
        bst.delete(&1);
        bst.delete(&2);
        // Agora a árvore está skewed-right (4, 5, 6). Altura deve ser 3.
        assert_eq!(bst.altura(), 3);
    }
}