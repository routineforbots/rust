//#[derive(Debug)]
enum TicketTypes {
    Backstage(String), // another option is to add both name and price into variant:  Backstage(String, f64)
    Standard, // same but only with price: Standrd(f64)
    Vip(String), // same here for another option: Vip(String, f64)
}

//#[derive(Debug)]
struct Ticket {
    ticket_type: TicketTypes,
    price: i32,
}


fn main() {
    let tickets  = vec! [
        Ticket{ticket_type: TicketTypes::Backstage("Alex".to_owned()),  price: 300},
        Ticket{ticket_type: TicketTypes::Vip("Mary".to_owned()), price: 1000},
        Ticket{ticket_type: TicketTypes::Standard, price: 100},
    ];

    for t in tickets {
        match t {
            Ticket{ticket_type: TicketTypes::Backstage(name), price} => println!("ticket type: Backstage, price: {:?}, for: {:}", price, name),
            Ticket{ticket_type: TicketTypes::Vip(name), price} => println!("ticket type: Vip, price: {:?}, for: {:}", price, name),
            Ticket{ticket_type: TicketTypes::Standard, price} => println!("ticket type: Standard, price: {:?}", price),
        }
    }
}