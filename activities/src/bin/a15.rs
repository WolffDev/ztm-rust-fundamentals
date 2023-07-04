// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price

enum Ticket {
    Backstage(f32, String),
    VIP(f32, String),
    Standard(f32, String),
}

//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
    let tickets = vec![
        Ticket::Standard(30.5, "Max".to_owned()),
        Ticket::VIP(40.5, "Caroline".to_owned()),
        Ticket::Backstage(50.5, "Luna".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Ticket belongs to {} at the price of {}", holder, price)
            }
            Ticket::VIP(price, holder) => {
                println!("Ticket belongs to {} at the price of {}", holder, price)
            }
            Ticket::Standard(price, holder) => {
                println!("Ticket belongs to {} at the price of {}", holder, price)
            }
        }
    }
}
