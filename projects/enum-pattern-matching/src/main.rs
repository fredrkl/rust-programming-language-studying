enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Hello, world again!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // Rust uses :: instead of . to access the enum variants and differentiate it from field access or method calls.
    //route(four);
    //route(six);
}

/*fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}*/
