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
