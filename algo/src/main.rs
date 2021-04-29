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
fn bubble_sort(mut arr: [i32; 7]) -> [i32; 7] {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    arr
}
fn selection_sort(mut arr: [i32; 7]) -> [i32; 7] {
    let mut min;
    for i in 0..arr.len() - 1 {
        min = i;
        for j in i + 1..arr.len() {
            // Finding the minimum element in the array
            if arr[j] < arr[min] {
                min = j;
            }
        }
        // Swap the found minimum element with the first element
        let temp = arr[i];
        arr[i] = arr[min];
        arr[min] = temp;
    }
    arr
}
fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 9];
    arr = selection_sort(arr);
    println!("Sorted array is {:?}", arr);
}
