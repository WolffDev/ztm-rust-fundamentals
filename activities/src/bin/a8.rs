// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Grape,
    Apple,
    Cherry,
    Strawberry,
}

struct Drink {
    flavor: Flavor,
    fluid_ounces: f64,
}

fn main() {
    let new_drink1 = Drink {
        flavor: Flavor::Orange,
        fluid_ounces: 12.0,
    };

    let new_drink2 = Drink {
        flavor: Flavor::Grape,
        fluid_ounces: 12.0,
    };

    print_drink(new_drink1);
    print_drink(new_drink2);
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("Orange"),
        Flavor::Grape => println!("Grape"),
        Flavor::Apple => println!("Apple"),
        Flavor::Cherry => println!("Cherry"),
        Flavor::Strawberry => println!("Strawberry"),
    }
    println!("{} ounces", drink.fluid_ounces);
}
