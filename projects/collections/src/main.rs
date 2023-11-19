fn main() {
    println!("Hello, world!");

    let _v: Vec<i32> = Vec::new();
    let _w = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let e = vec![1, 2, 3, 4, 5];
    let third = &e[2];
    // e.push(6); // Changing e to mutable (mut) and uncommenting this line will cause an error because we are trying to borrow a mutable reference to e while we have an immutable reference to e
    println!("The third element is {}", third);

    let third = e.get(2); // The reson why this is option is because it might not exist, the index might be out of bounds
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}
