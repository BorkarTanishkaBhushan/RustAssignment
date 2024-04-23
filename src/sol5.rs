// Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    if len % 2 == 0 {
        let mid = len / 2;
        let avg: f64 = sorted_arr[mid - 1] as f64 + sorted_arr[mid] as f64 / 2.0;
        return avg;
    } else {
        let mid = len / 2;
        return sorted_arr[mid] as f64;
    }
}

fn main() { 
    let unsorted_array = [8, 6, 2, 7, 1, 8, 4, 9, 3]; 
    let median = find_median(&unsorted_array);
    println!("Sorted Array: {:?}", unsorted_array);
    println!("Median: {}", median);
}
