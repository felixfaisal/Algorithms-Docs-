pub struct Student {
    pub name: String,
    pub age: u16,
}
impl Student {
    pub fn display(&self) {
        println!("Name: {} \nAge: {}", self.name, self.age);
    }
    pub fn clone(&self) -> Student {
        Student {
            name: String::from(&self.name),
            age: self.age,
        }
    }
}
pub fn bubble_sort(mut arr: [Student; 4]) -> [Student; 4] {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j].age > arr[j + 1].age {
                let temp = arr[j].clone();
                arr[j] = arr[j + 1].clone();
                arr[j + 1] = temp.clone();
            }
        }
    }
    arr
}
