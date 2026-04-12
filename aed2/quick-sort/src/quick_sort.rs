use std::usize;


pub trait QuickSortable {
    ///
    fn qsort(&mut self);
    fn qsort_naive(&mut self);
}

impl<T> QuickSortable for &mut [T] 
where T: Ord + Copy
{
    fn qsort(&mut self) {

        qsort_recursive(self, 0, self.len() - 1);
    }
    
    fn qsort_naive(&mut self) {
        
        qsort_recursive_naive(self, 0, self.len() - 1);
    }


}

fn qsort_recursive<T> (slice: &mut [T], start: usize, end: usize) 
where T: Ord + Copy
{
    if start >= end || end > slice.len() {
        return;
    }

    let pi = partition(slice, start, end);

    qsort_recursive(slice, start, usize::wrapping_sub(pi, 1));
    qsort_recursive(slice, pi + 1, end);
}

/// 
fn partition<T> (slice: &mut [T], start: usize, end: usize) -> usize
where T: Ord + Copy
{   
    
    move_pivot_to_end(slice, start, end);

    let pivot = slice[end];

    let mut i: usize = start; //

    for j in start..=end {
        if slice[j] <= pivot {
            slice.swap(i, j); 
            i += 1;                
        }
    }

    if i > 0 {i -= 1};

    // slice.swap(i + 1, end);
    return i;
}

/// Seleciona três candidatos a pivot: O primeiro elemento, o do meio e o último
/// Pega o elemento da mediana entre os três e o coloca no final do slice
fn move_pivot_to_end<T> (slice: &mut [T], start: usize, end: usize)
where T: Ord + Copy 
{
    let middle = (start + end) / 2;

    let a: T = slice[start];
    let b: T = slice[middle];
    let c: T = slice[end];

    if a > b {
        slice.swap(start, middle);
    }
    if a > c {
        slice.swap(start, end);
    }
    if b > c {
        slice.swap(middle, end);
    }

    // Agora a mediana está em middle — move para end
    slice.swap(middle, end);
}

fn qsort_recursive_naive<T> (slice: &mut [T], start: usize, end: usize) 
where T: Ord + Copy
{
    if start >= end {
        return;
    }

    let pi = partition_naive(slice, start, end);

    if pi > 0 {qsort_recursive_naive(slice, start, pi - 1);}
    qsort_recursive_naive(slice, pi + 1, end);
}

fn partition_naive<T> (slice: &mut [T], start: usize, end: usize) -> usize
where T: Ord + Copy
{   
    let pivot = slice[end];

    let mut i: usize = start; //

    for j in start..=end {
        if slice[j] <= pivot {
            slice.swap(i, j); 
            i += 1;                
        }
    }

    if i > 0 {i -= 1};

    // slice.swap(i + 1, end);
    return i;
}