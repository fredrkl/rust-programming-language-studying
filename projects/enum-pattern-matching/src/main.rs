enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32), 
}

impl Message {
   fn call(&self){
    println!("Now we are printing out");
   } 
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn main() {
    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));
    println!("Hello, world again!");

    let _something = Message::Move { x: 34, y: 45 };
    let _somethingelse = Message::Write(String::from("Hello, world!"));

    _somethingelse.call();
    // Rust uses :: instead of . to access the enum variants and differentiate it from field access or method calls.
}
