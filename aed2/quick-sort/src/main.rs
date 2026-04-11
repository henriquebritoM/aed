mod quick_sort;
use quick_sort::QuickSortable;

fn main() {

    let mut arr = [1, 56, 15, 6, 4, 87, 1, 68, 7, 9, 46, 168, 79, 
    4, 61, 94, 64, 53, 96, 45, 16, 53];

    arr.as_mut_slice().qsort();

    println!("{:#?}", arr);
    
    assert!(arr.is_sorted());
}
