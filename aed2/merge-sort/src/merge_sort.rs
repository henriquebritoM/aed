
/// Trait applied to all vectors, slices, arrays, ...
/// Gives them the method .merge_sort().
pub trait MergeSortable<T> 
where T: Ord + Copy
{
    fn merge_sort(&mut self);     
}

impl<T> MergeSortable<T> for &mut [T] 
where T: Ord + Copy
{
    /// Calls merge sort as method
    fn merge_sort(&mut self) {
        
        if self.len() <= 1 {
            return;
        }
    
        let mid = self.len() / 2;
    
        let (mut left, mut right) = self.split_at_mut(mid);
    
        left.merge_sort();
        right.merge_sort();

        merge(left, right);
    }
}

/// Merge left and right ordenadamente, como se fossem continuas em memória.
/// 
/// Exemplo:
/// antes do merge
/// left: [5, 6, 7], right: [1, 2, 3]
/// após o merge
/// left: [1, 2, 3], right: [5, 6, 7]
fn merge<T> (left: &mut [T], right: &mut [T]) 
where T: Ord + Copy 
{   
    let mut result: Vec<T> = Vec::with_capacity(left.len() + right.len());

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;

    while i < left.len() && j < right.len() {

        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        }
        else {
            result.push(right[j]);
            j += 1;
        }
    }

    //  Termina re preencher result com os elementos de left e right
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    //  Preenche novamente os slices originais, mas ordenadamente
    while k < left.len() {
        left[k] = result[k];
        k += 1;
    }

    k = 0;

    while k < right.len() {
        right[k] = result[k + left.len()];
        k += 1;
    }
}


