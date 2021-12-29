#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { 
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
   
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle = &Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rectangle.area());
    println!("rectangle is {:#?}", rectangle);

    let rect2 = Rectangle::square(10);

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println!("Can rectangle hold rect2? {}", rectangle.can_hold(&rect2));
    println!("Can rectangle hold rect3? {}", rectangle.can_hold(&rect3));
}
