// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
struct Shoes(Color);

impl Shoes {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Shirt(Color);

impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct Pants(Color);

impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

fn main() {
    let shoes = Shoes::new(Color::Black);
    let shirt = Shirt::new(Color::Red);
    let pants = Pants::new(Color::Blue);

    println!("{:?}", shoes);
    println!("{:?}", shirt);
    println!("{:?}", pants);
}
