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
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
