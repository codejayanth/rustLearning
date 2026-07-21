enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAdd {
    V4(String),
    V6(String),
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let office = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    // Another way to bypass the structs is
    let home_two = IpAdd::V4(String::from("127.0.0.1"));
    let office_two = IpAdd::V6(String::from("::1"));
}

// fn route(ip_addr_kind: IpAddrKind) {}
