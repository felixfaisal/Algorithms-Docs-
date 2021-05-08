mod sort;
// Initializing a Structure
mod studentstruct;
use studentstruct::Student;
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
    students = studentstruct::bubble_sort(students);
    for x in students.iter() {
        x.display()
    }
}
