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

    let _x : u16 = 1_000;
    let _mybyte : u8 = 0b1111_0000;
    println!("The value of _mybyte is now: {_mybyte}");

    // Floating point types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    println!("The value of _x is now: {_x}");
    println!("The value of _y is now: {_y}");

    // Char type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    println!("The value of _heart_eyed_cat is now: {_heart_eyed_cat}");

    // Tuple type
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;
    println!("The value of _y is: {_y}");
    let b = _tup.1;
    println!("The value of second _tup is: {b}");
    let _c = ();

    // Array type. Arrays in Rust have a fixed length, like tuples.
    let _a:[u8;5] = [1, 2, 3, 4, 5];
    let _months = ["january", "february", "march", "april", "may", "june", "july",
                  "august", "september", "october", "november", "december"];
}
