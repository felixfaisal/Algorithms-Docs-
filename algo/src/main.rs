mod sort;
// Initializing a Structure
struct Student {
    name: String,
    age: u16,
}
impl Student {
    fn display(&self) {
        println!("Name: {} \nAge: {}", self.name, self.age);
    }
    fn clone(&self) -> Student {
        Student {
            name: String::from(&self.name),
            age: self.age,
        }
    }
}
fn bubble_sort(mut arr: [Student; 4]) -> [Student; 4] {
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
fn main() {
    let mut students: [Student; 4] = [
        Student {
            name: String::from("Felix"),
            age: 21,
        },
        Student {
            name: String::from("Mary"),
            age: 17,
        },
        Student {
            name: String::from("Oliver"),
            age: 16,
        },
        Student {
            name: String::from("Barry"),
            age: 25,
        },
    ];
    students = bubble_sort(students);
    for x in students.iter() {
        x.display()
    }
}
