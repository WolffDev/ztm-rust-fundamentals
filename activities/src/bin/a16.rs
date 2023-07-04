// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Locker {
    name: String,
    number: Option<i32>,
}

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

fn main() {
    let student = Locker {
        name: "Max".to_owned(),
        number: Some(42),
    };
    println!("Student name: {:?}", student.name);
    match student.number {
        Some(number) => println!("Number is: {}", number),
        None => println!("No locker number assigned"),
    }
}
