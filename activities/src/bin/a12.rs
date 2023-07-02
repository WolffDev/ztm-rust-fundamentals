// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    color: Color,
    weight: u32,
    dimensions: Dimensions,
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Black,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Blue => println!("Blue"),
            Color::Yellow => println!("Yellow"),
            Color::Orange => println!("Orange"),
            Color::Purple => println!("Purple"),
            Color::Black => println!("Black"),
            Color::White => println!("White"),
        }
    }
}

struct Dimensions {
    length: u32,
    width: u32,
    height: u32,
}

impl Dimensions {
    fn print(&self) {
        println!(
            "Dimensions: {} x {} x {}",
            self.length, self.width, self.height
        );
    }
}

impl Box {
    fn new(color: Color, weight: u32, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        println!("Weight: {} lbs", self.weight);
        self.dimensions.print();
    }
}

fn main() {
    let box1 = Box::new(
        Color::Red,
        10,
        Dimensions {
            length: 10,
            width: 10,
            height: 10,
        },
    );
    box1.print();
}
