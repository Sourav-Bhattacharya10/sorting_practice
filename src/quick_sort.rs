use crate::util::is_sorted;

pub fn quick_sort(num_array: Vec<u8>) -> Vec<u8> {
    if num_array.len() <= 1 {
        return num_array;
    }

    // Check if the array is already sorted
    if is_sorted(&num_array) {
        return num_array;
    }

    let last_index = num_array.len() - 1;
    let pivot = num_array[last_index];

    let mut left_array: Vec<u8> = vec![];
    let mut right_array: Vec<u8> = vec![];

    for val in 0..num_array.len() - 1 {
        if num_array[val] < pivot {
            left_array.push(num_array[val]);
        } else {
            right_array.push(num_array[val]);
        }
    }

    let sorted_left_array = quick_sort(left_array);
    let sorted_right_array = quick_sort(right_array);

    [sorted_left_array, vec![pivot], sorted_right_array].concat()
}
