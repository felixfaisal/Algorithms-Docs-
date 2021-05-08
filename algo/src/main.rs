mod sort;
// Initializing a Structure
struct Student {
    name: String,
    age: u16,
}
fn main() {
    let faisal = Student {
        name: String::from("Faisal"),
        age: 21,
    };
    let students: [Student; 2] = [
        Student {
            name: String::from("Felix"),
            age: 21,
        },
        Student {
            name: String::from("Mary"),
            age: 21,
        },
    ];
    println!("Name: {} \nAge: {}", faisal.name, faisal.age);
}
