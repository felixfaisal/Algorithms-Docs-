fn binary_search(key: i32, arr: [i32; 9]) {
    let mut left = 0;
    let mut right = arr.len() - 1;
    while right >= left {
        let mid = left + (right - left) / 2; // Defining the mid element
        if arr[mid] == key {
            // If the element is present at mid itself
            println!("Element found at index {}", mid);
            break;
        } else if arr[mid] > key {
            // If mid is greater, Then moving to the left sub array
            right = mid - 1;
        } else {
            // If mid is lower, Then moving to the right sub array
            left = mid + 1;
        }
    }
    if left > right {
        println!("Element is not present in the array")
    }
}
fn linear_search(key: i32, arr: [i32; 9]) {
    let mut flag = false;
    for item in 0..arr.len() - 1 {
        if arr[item] == key {
            // element found in the array
            println!("Element found at index {} ", item);
            flag = true;
            break;
        }
    }
    if flag == false {
        // Meaning element is not present in the array
        println!("Element not present in the array");
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // Sorted array
    linear_search(20, arr);
    linear_search(8, arr);
}
