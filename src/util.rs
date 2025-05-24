// Function to check if an array is sorted
pub fn is_sorted(arr: &Vec<u8>) -> bool {
    for i in 0..arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}