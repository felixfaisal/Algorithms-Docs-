mod sort;
fn partition(mut arr: [i32; 7], left: usize, right: usize) -> (usize, [i32; 7]) {
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

fn quick_sort(mut arr: [i32; 7], left: usize, right: usize) -> [i32; 7] {
    if left < right {
        // let pi: usize;
        let (pi, x) = partition(arr, left, right);
        arr = x.clone();
        arr = quick_sort(arr, left, pi - 1); // Before pi
        arr = quick_sort(arr, pi, right); // After pi
    }
    arr
}
fn main() {
    let mut arr = [64, 34, 25, 8, 22, 11, 9];
    // arr = sort::merge_sort(arr, 0, arr.len());
    arr = quick_sort(arr, 0, arr.len() - 1);
    println!("Sorted array is {:?}", arr);
}
