---
sidebar_position: 1
---
# Linear Search
:::tip Important 

Algorithm implemented in Rust

:::

**Linear search** is rarely used practically because other search algorithms such as the binary search algorithm and hash tables allow significantly faster-searching comparison to Linear search.

The time complexity of the linear search algorithm is `O(n)`. 

## Pseudocode

``` 
Step 1:- Start from the first element and iterate through the array 
Step 2:- If element is found then break the loop 
Step 3:- If loop completes, Print element not found
``` 

## Code 

```rust
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
``` 

## Output 

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/algo`
Element not present in the array
Element found at index 7  
``` 