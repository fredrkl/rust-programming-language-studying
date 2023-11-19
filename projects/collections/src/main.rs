fn main() {
    println!("Hello, world!");

    let _v: Vec<i32> = Vec::new();
    let _w = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    // e.push(6); // Changing e to mutable (mut) and uncommenting this line will cause an error because we are trying to borrow a mutable reference to e while we have an immutable reference to e
    println!("The third element is {}", third);

    let third = v.get(2); // The reson why this is option is because it might not exist, the index might be out of bounds
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &v {
        println!("{i}")
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Dereference i to get the value it refers to, then add 50 to that value
    }

    for i in &v {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("blue")), SpreadsheetCell::Float(10.12)];
    for i in &row {
        match i {
            SpreadsheetCell::Text(i) => println!("{i}"),
            SpreadsheetCell::Float(i) => println!("{i}"),
            SpreadsheetCell::Int(i) => println!("{i}")
        }
    }

    let s = String::new();
    let _reference = &s;
    let data = "initial contents";
    let mut _s = data.to_string();
    let s2 = "initial contents";
    _s.push_str(s2);
    println!("{s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // When operating on strings do not use indexing because it is not guaranteed to work with all languages as excepted
    // This is the proper way to do it
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
