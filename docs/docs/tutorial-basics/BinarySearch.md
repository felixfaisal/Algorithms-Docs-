---
sidebar_position: 6
---

# Binary Search 
:::tip Important 

Algorithm implemented in Rust

:::

Search a sorted array by repeatedly dividing the search interval in half. Begin with an interval covering the whole array. If the value of the search key is less than the item in the middle of the interval, narrow the interval to the lower half. Otherwise, narrow it to the upper half. Repeatedly check until the value is found or the interval is empty.

The time complexity of the binary search algorithm is `O(log n)`. The best-case time complexity would be O(1) when the central index would directly match the desired value. The worst-case scenario could be the values at either extremity of the list or values not in the list.

:::danger Important 

The array needs to be sorted to perform this algorithm

:::

## Pseudocode

```
Step 1 :- Find the middle element 
Step 2 :- Compare the middle element with the key 
Step 3 :- Move to the right or left side of the array 
Step 4 :- Repeat the process until you find the key 
``` 
## Code 

```rust
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

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9]; // Sorted array
    binary_search(20, arr);
    binary_search(8, arr);
}
```

## Output 

```rust
   Compiling algo v0.1.0 (/home/felix/Documentation/Algorithms-Docs-/algo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/algo`
Element is not present in the array
Element found at index 7
```


