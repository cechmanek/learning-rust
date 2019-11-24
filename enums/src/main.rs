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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // skip the rest
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // add a type for specially minted quarters
}

// this function takes immutable reference to coin
fn value_in_cents(coin: &Coin) -> u8 {
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // inside the 'match' block we can access enum sub values
           println!("We found a special quarter from state: {:?}", state); // need :? to print special enum value
           25 // same as 'return 25;'
        },
    };
    println!("value is {}", value);
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
    value_in_cents(&my_coin);

    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    
    value_in_cents(&alabama_quarter);
}
