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

// we can use the optional<T> enum when we need optionally present values
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // return Option::None if that is what we were passed in
        Some(i) => Some(i+1), // increment by 1 if we got a non-None. 'i' is local argument
    }
    // this function returns the result of match as it's the last value with no semi-colon
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

    //using Option<T> to handle None values

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five is: {:?}", five);
    println!("six is: {:?}", six);
    println!("none is: {:?}", none);
   
    // match blocks must be exhaustive, but there is a catch-all '_'
    let my_num :u8 = 5;

    let the_same_num = match my_num {
        0 => "zero",
        1 => "one",
        2 => "two",
        4 => "four",
        8 => "eight",
        _ => "not 0, 1, 2, 4, or 8",
    };

    println!("my_num is {}, aka {}", my_num, the_same_num);


    // match blocks must be exhaustive, but there is a catch-all '_'
    let my_num :u8 = 4;

    let the_same_num :String = match my_num {
        0 => "zero",
        1 => "one",
        2 => "two",
        4 => "four",
        8 => "eight",
        _ => "not 0, 1, 2, 4, or 8",
    };

    println!("my_num is {}, aka {}", my_num, the_same_num);


}
