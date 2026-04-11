use crate::merge_sort::MergeSortable;

mod merge_sort;

fn main() {

     let mut arr = [1, 56, 15, 6, 4, 87, 1, 68, 7, 9, 46, 168, 79,
    4, 61, 94, 64, 53, 96, 45, 16, 53];

    arr.as_mut_slice().merge_sort();

    println!("{:#?}", arr);

    assert!(arr.is_sorted());

}
