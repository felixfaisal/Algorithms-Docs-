---
sidebar_position: 4
---
# Selection Sort

The selection sort algorithm sorts an array by repeatedly finding the minimum element (considering ascending order) from unsorted part and putting it at the beginning. The algorithm maintains two subarrays in a given array.

1) The subarray which is already sorted.

2) Remaining subarray which is unsorted.

In every iteration of selection sort, the minimum element (considering ascending order) from the unsorted subarray is picked and moved to the sorted subarray.

## Code 

```rust
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
``` 

## Output 

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/algo`
Sorted array is [9, 11, 12, 22, 25, 34, 64]
```