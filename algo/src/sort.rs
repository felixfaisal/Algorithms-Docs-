pub fn binary_search(key: i32, arr: [i32; 9]) {
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
pub fn linear_search(key: i32, arr: [i32; 9]) {
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
pub fn bubble_sort(mut arr: [i32; 7]) -> [i32; 7] {
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
pub fn selection_sort(mut arr: [i32; 7]) -> [i32; 7] {
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

pub fn read_cli() {
    let mut line = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
}
pub fn merge_sort(mut arr: [i32; 7], left: usize, right: usize) -> [i32; 7] {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}
fn merge(mut arr: [i32; 7], left: usize, mid: usize, right: usize) -> [i32; 7] {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut L1 = arr.clone();
    let mut R1 = arr.clone();
    let L = &L1[left..mid];
    let R = &R1[mid..right];
    /* Merge the temp arrays back into arr[l..r]*/
    let mut i = 0; // Initial index of first subarray
    let mut j = 0; // Initial index of second subarray
    let mut k = left; // Initial index of merged subarray
                      // println!("Left subarry {:?}", &L[left..mid]);
                      // println!("Right subarry {:?}", &R[mid + 1..right]);
    while i < n1 && j < n2 {
        if L[i] < R[j] {
            arr[k] = L[i];
            i = i + 1;
        } else {
            arr[k] = R[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < n1 {
        arr[k] = L[i];
        i = i + 1;
        k = k + 1;
    }
    /* Copy the remaining elements of R[], if there
    are any */
    while j < n2 {
        arr[k] = R[j];
        j = j + 1;
        k = k + 1;
    }
    arr
}
pub fn partition(mut arr: [i32; 7], left: usize, right: usize) -> (usize, [i32; 7]) {
    // pivot (Element to be placed at right position)
    let pivot = arr[right];
    let leftI = left as i32;
    let mut i: i32 = leftI - 1;
    for j in left..right {
        if arr[j] < pivot {
            i = i + 1; // increment index of smaller element
            let temp = arr[i as usize];
            arr[i as usize] = arr[j];
            arr[j] = temp; // swap arr[i] and arr[j]
        }
    }
    // swap arr[i + 1] and arr[high])
    let finalpos = i + 1;
    let temp = arr[right];
    arr[right] = arr[finalpos as usize];
    arr[finalpos as usize] = temp;
    (finalpos as usize, arr)
}

pub fn quick_sort(mut arr: [i32; 7], left: usize, right: usize) -> [i32; 7] {
    if left < right {
        // let pi: usize;
        let (pi, x) = partition(arr, left, right);
        arr = x.clone();
        arr = quick_sort(arr, left, pi - 1); // Before pi
        arr = quick_sort(arr, pi, right); // After pi
    }
    arr
}
