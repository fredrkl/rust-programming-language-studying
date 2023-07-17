fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1.push_str(", world!");
    println!("{}", s2);

    let s = String::from("hello");
    takes_ownership(s.clone());
    println!("{}", s); // This works because s is cloned into the takes_ownership method
    takes_ownership(s);
    // println!("{}", s); // This doesn't work because s is moved into the takes_ownership method

    let x = 5;
    takes_ownership_int(x);
    println!("{}", x); // This works because i32 is a Copy type

    let x = String::from("hello");
    let x2 = takes_ownership_and_gives_back(x);
    println!("{}", x2); // This works because x is returned from the takes_ownership_and_gives_back method

    let x = String::from("hello");
    let length = calculate_length(&x);
    println!("The length of {} is {}.", x, length);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // Have multiple mutable references to the same data is not allowed
    let mut s = String::from("hello");
    {
        let r2 = &mut s;
        println!("{}", r2);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
    let r1 = &mut s;
    println!("{}", r1);

    let s = String::from("first word");
    let word = first_word(&s);
    println!("{}", word);

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("{}", word);

    arrary_slices_demo();
}

fn arrary_slices_demo() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership_and_gives_back(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership_int(some_int: i32) {
    println!("{}", some_int);
}
