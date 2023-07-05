fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;

    {
        let x = x + 1;
        println!("The value of x inner-scope is: {x}");
    }

    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}
