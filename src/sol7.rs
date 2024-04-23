//Implement a function that returns the kth smallest element in a given array.

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; 
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    return Some(sorted_arr[k - 1])
}

fn main() {
    let arr = [9, 50, 3, 3, 5, 35];
    let k = 3;
    if let Some(kth_smallest) = kth_smallest_element(&arr, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Invalid value of k: {}", k);
    }
}
