#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing the area method on the Rectangle struct
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn height(&self) -> u32 {
        self.height
    } 
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: dbg!(20 * scale),
        height: 40,
    };

    dbg!(&rect1);
    //println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The width of the rectangle is larger than 0 {}: ", rect1.width());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Square: {:#?}", Rectangle::square(10));
    println!("Height: {}", rect1.height());

    let config_max = Some(3u8);

}

/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}*/
