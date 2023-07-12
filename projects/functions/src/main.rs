fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(5);
    
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
