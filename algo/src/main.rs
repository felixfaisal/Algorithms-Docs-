fn main() {
    println!("Binary Search!");
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("The array is {:?}", arr);
    println!("The length of the array is: {}", arr.len());
    let mut mid = arr.len() / 2;
    println!("The middle element index is {}", mid);
    println!("The middle element is {}", arr[mid]);
    for item in 0..arr.len() {
        println!("{}", arr[item]);
    }
}
