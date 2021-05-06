mod sort;
fn partition(arr:[i32;7],left:i32,right:i32) -> {
    
}
fn quick_sort(arr:[i32;7],left:i32,right:i32) -> {
    if low<high{
        let pi = partition(arr,low,high);
    }
}
fn main() {
    let mut arr = [64, 34, 25, 8, 22, 11, 9];
    arr = sort::merge_sort(arr, 0, arr.len());
    println!("Sorted array is {:?}", arr);
}
