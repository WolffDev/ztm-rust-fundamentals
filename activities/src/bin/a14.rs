// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Max"),
            age: 37,
            favorite_color: String::from("Green"),
        },
        Person {
            name: String::from("Caroline"),
            age: 37,
            favorite_color: String::from("Red"),
        },
        Person {
            name: String::from("Luna"),
            age: 37,
            favorite_color: String::from("Blue"),
        },
    ];

    for person in people {
        if person.age > 20 {
            print_name_color(&person);
        }
    }
}

fn print_name_color(person: &Person) {
    println!("Name: {}, Color: {}", person.name, person.favorite_color)
}
