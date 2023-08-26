#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing the area method on the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
    //println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The width of the rectangle is larger than 0 {}: ", rect1.width());
}

/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}*/
