mod sort;
fn main() {
    let mut arr = [64, 34, 25, 8, 22, 11, 9];
    arr = sort::merge_sort(arr, 0, arr.len());
    println!("Sorted array is {:?}", arr);
}
