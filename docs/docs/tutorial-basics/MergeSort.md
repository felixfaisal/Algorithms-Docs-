---
sidebar_position: 3
---
# Bubble Sort

**Bubble Sort** is the simplest sorting algorithm that works by repeatedly swapping the adjacent elements if they are in wrong order.

## Code 

```rust
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
fn main() {
    let mut arr = [64, 34, 25, 12, 22, 11, 9];
    arr = bubble_sort(arr);
    println!("Sorted array is {:?}", arr);
} 
``` 

## Output 

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/algo`
Sorted array is [9, 11, 12, 22, 25, 34, 64] 
```