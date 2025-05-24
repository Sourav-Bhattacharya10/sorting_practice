// mod merge_sort;
mod quick_sort;
mod util;

// use merge_sort::merge_sort;
use quick_sort::quick_sort;

fn main() {
    let num_array: Vec<u8> = vec![3,1,2,1,2];
    // println!("Merge Sorted Array: {:?}", merge_sort(num_array.clone()));
    println!("Quick Sorted Array: {:?}", quick_sort(num_array));
}
