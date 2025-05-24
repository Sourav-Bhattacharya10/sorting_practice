use crate::util::is_sorted;

pub fn merge_sort(num_array: Vec<u8>) -> Vec<u8> {
    if num_array.len() <= 1 {
        return num_array;
    }

    // Check if the array is already sorted
    if is_sorted(&num_array) {
        return num_array;
    }

    let mid_index = num_array.len() / 2;
    let left_array = merge_sort(num_array[0..mid_index].to_vec());
    let right_array = merge_sort(num_array[mid_index..].to_vec());

    merge(left_array, right_array)
}

fn merge(left_array: Vec<u8>, right_array: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < left_array.len() && j < right_array.len() {
        if left_array[i] < right_array[j] {
            result.push(left_array[i]);
            i += 1;
        } else {
            result.push(right_array[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left_array[i..]);
    result.extend_from_slice(&right_array[j..]);

    result
}
