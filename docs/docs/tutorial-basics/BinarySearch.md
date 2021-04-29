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

```
Step 1 :- Find the middle element 
Step 2 :- Compare the middle element with the key 
Step 3 :- Move to the right or left side of the array 
Step 4 :- Repeat the process until you find the key 
``` 