fn main() {
    let mut x = 5;
    let y = 5;
    println!("The value of y is: {y}");
    println!("The value of x is: {x}");

    x = 6;
    {
        let y = y + 1;
        println!("The value of y inner-scope is: {y}");
    }

    println!("The value of x is now: {x}");
    
    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}
