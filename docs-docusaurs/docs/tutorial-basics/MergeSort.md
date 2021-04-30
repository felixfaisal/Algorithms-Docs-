---
sidebar_position: 4
---

# Merge Sort

Like QuickSort, Merge Sort is a **Divide and Conquer** algorithm. It divides the input array into two halves, calls itself for the two halves, and then merges the two sorted halves. The merge() function is used for merging two halves. The `merge(arr, l, m, r)` is a key process that assumes that arr[l..m] and arr[m+1..r] are sorted and merges the two sorted sub-arrays into one

## Pseudocode 

``` 
MergeSort(arr[], l,  r)
If r > l
     1. Find the middle point to divide the array into two halves:  
             middle m = l+ (r-l)/2
     2. Call mergeSort for first half:   
             Call mergeSort(arr, l, m)
     3. Call mergeSort for second half:
             Call mergeSort(arr, m+1, r)
     4. Merge the two halves sorted in step 2 and 3:
             Call merge(arr, l, m, r)
```

## Code 

```rust
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
fn merge_sort(mut arr: [i32; 7], left: usize, right: usize) -> [i32; 7] {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort(arr, left, mid);
        arr = merge_sort(arr, mid, right);
        arr = merge(arr, left, mid, right);
    }
    arr
}

fn main() {
    let mut arr = [64, 34, 25, 8, 22, 11, 9];
    arr = merge_sort(arr, 0, arr.len());
    println!("Sorted array is {:?}", arr);
}
``` 

## Output 

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/algo`
Sorted array is [8, 9, 11, 22, 25, 34, 64]
```