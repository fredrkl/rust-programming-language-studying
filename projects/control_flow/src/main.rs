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
    count_down();
    for_loop_demo();
    count_down_using_for();
}

fn count_down_using_for() {
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn count_down() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop_demo() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {}", element);
    }
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
