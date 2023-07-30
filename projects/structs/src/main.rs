fn main() {
    println!("Hello, world again!");

    let mut person = User {
        email: String::from("appelsin@eple.no"),
        username: String::from("appelsin"),
        active: true,
        sign_in_count: 1,
    };

    person.email = String::from("another@email.no");
    let another_person = build_user(String::from("anemail@mail.no"), String::from("a user"));
    println!("another person: {}", another_person.email);

    let shorthand_person = build_user_2(String::from("the_email@email.no"),
                                        String::from("the_username"));
    println!("shorthand person: {}", shorthand_person.email);

    let person3 = User {
        email: String::from("another"),
        ..another_person
    };
    println!("person3 email: {}", person3.email);
    println!("person3 username: {}", person3.username);
    println!("person3 active: {}", person3.active);
    println!("person3 sign_in_count: {}", person3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("point: {}, {}, {}", origin.0, origin.1, origin.2);

    let _subnect = AlwaysEqual;
    let something = String::from("something");
    string_demo(&something);
    println!("something: {}", something);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn string_demo(astring: &str){
    println!("{}", astring);

}

fn build_user_2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct AlwaysEqual;
// How is the ownership of a struct's data determined?
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
