//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1; 
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = vec![1, 2, 3, 4, 4, 4, 5, 6, 7, 8];
    let target = 4;
    if let Some(index) = first_occurrence(&arr, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}