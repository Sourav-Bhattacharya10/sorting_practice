mod merge_sort;
// mod quick_sort;

// use quick_sort::quick_sort;
use merge_sort::merge_sort;

fn main() {
    let num_array: Vec<u8> = vec![3, 1, 2]; // , 2, 1
    // let mid_index = num_array.len() / 2;
    // println!("Mid-index : {}", mid_index);
    // println!("Quick Sorted array : {:?}", quick_sort(num_array));
    println!("Merge Sorted array : {:?}", merge_sort(num_array));
}
