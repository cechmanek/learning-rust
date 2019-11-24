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

fn main() {
    let home = IpAddress::V4(127,0,0,1);
    
    let loopback = IpAddress::V6(String::from("::1"));

    // both V4 and V6 have the same parent type so can be consumed by the same function
    route(&home);
    route(&loopback);

    // calling methods of enums the same as we do from structs
    home.route();
    loopback.route();
}
