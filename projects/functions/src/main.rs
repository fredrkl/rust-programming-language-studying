fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    print_labeled_measurement(5, 'm'); 
    println!("The value of five is: {}", five());
    println!("The value of plus_one(5) is: {}", plus_one(5));
}

fn another_function(){
    println!("Another function");
}

fn another_function_with_parameter(x: i32){
    println!("Another function with parameter. The argument of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The value is: {} {}", value, unit_label);
}

fn five () -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
