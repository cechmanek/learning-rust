enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddress {
    fn route(&self) {
        println!("Can define methods attached to enums the same as we do structs");
    }
}

fn route(ip_kind: &IpAddress) {
    // empty function 
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// this function takes immutable reference to coin
fn print_value_in_cents(coin: &Coin) {
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("value is {}", value);
}

// this function takes ownership of coin
fn get_value_in_cents(coin: Coin) -> u8 {
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25, 
        // all enum possibilities must exist. 
        //comment out one line and compiler complains
    };
    return value;
}

fn main() {
    let home = IpAddress::V4(127,0,0,1);
    
    let loopback = IpAddress::V6(String::from("::1"));

    // both V4 and V6 have the same parent type so can be consumed by the same function
    route(&home);
    route(&loopback);

    // calling methods of enums the same as we do from structs
    home.route();
    loopback.route();

    // enums combined with 'match' keyword
    let my_coin = Coin::Nickel;

    print_value_in_cents(&my_coin);

    let value = get_value_in_cents(my_coin);

    println!("getting value of my coin: {}", value);

}
