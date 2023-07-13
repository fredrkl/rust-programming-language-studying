fn main() {
    println!("Hello, world!");
    let x = if true { 5 } else { 6 };
    // let y = if true { 5 } else { "a string" }; // error[E0308]: `if` and `else` have incompatible types
    println!("x is {}", x);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {}", result);
    inner_loop();
}

fn inner_loop() {
    let mut counter = 0;
    'counting_up: loop {
        println!("counter is {}", counter);
        let mut remaining = 10;
        loop {
            println!("remaining is {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }
}
