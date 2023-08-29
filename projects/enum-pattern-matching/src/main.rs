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

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // enum inside an enum
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(value) => Some(value + 1),
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
    let _money = Coin::Penny;
    value_in_cents(_money);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    let _five = Some(5);
    println!("The value of five is {:?}", _five);
    let _none = plus_one(None);
    println!("The value of none is {:?}", _none);
}
