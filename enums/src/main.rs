enum IpAddressKind {
    V4,
    V6,
}

struct IpAddress {
    kind : IpAddressKind,
    address : String,
}

fn route(ip_kind: IpAddressKind) {

}

fn main() {

    let my_ip = IpAddressKind::V4;

    // both V4 and V6 have the same parent type so can be consumed by the same function
    route(IpAddressKind::V6);
    route(my_ip);

    let home = IpAddress {
        kind : IpAddressKind::V4,
        address : String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind : IpAddressKind::V6,
        address : String::from("::1"),
    };

}
