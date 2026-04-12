
pub trait CountingSortable {
    fn counting_sort(&mut self);
}

//  Ord: Elementos podem ser ordenados fortemente (não inclui floating point)
//  Copy: Elementos não contém ponteiros para o Heap (não precisa de deep copy)
//  Into<usize> Elementos podem ser convertidos para usize 
impl<T> CountingSortable for [T] 
where T: Ord + Copy + TryInto<usize> + TryFrom<usize>
{
    fn counting_sort(&mut self) {
        
        let Some(max) = self.iter().max().cloned() else {
            return;
        };

        let Ok(max) = max.try_into() else {
            panic!("O tipo não pode ser convertido para usize");
        };

        let mut aux_vec: Vec<usize> = vec![0; max + 1];

        //  Calcula a quantidade de ocorrencias
        for i in 0..self.len() {
            let Ok(indice) = self[i].try_into() else {
                panic!("O tipo não pode ser convertido para usize");
            };
            aux_vec[indice] += 1;
        }

        //  Calcula as somas cumulativas
        let mut cont = 0;

        for i in 0..max + 1 {
            aux_vec[i] += cont; 
            cont = aux_vec[i];
        }

        //  Inicializa o vetor com o primeiro elemento, para não ficar com lixo
        //  Alguns type annotations horríveis aqui
        let mut result: Vec<T> = vec![self[0]; self.len()];
        for i in 0..self.len() {

            let Ok(el) = self[i].try_into() else {
                panic!("O tipo não pode ser convertido para usize");
            };

            let indice = aux_vec[el] - 1;
            aux_vec[el] -= 1;

            result[indice] = self[i];
        }

        self.swap_with_slice(&mut result);  //  Clean pointer swap
    }
}

