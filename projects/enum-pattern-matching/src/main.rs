enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world again!");
    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));
    // Rust uses :: instead of . to access the enum variants and differentiate it from field access or method calls.
    //route(four);
    //route(six);
}

/*fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}*/
