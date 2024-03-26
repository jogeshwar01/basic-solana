#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods - contain the &self as the first argument
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associated Functions = similar to static fns
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn get_area() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!(
        "The area of the square is {} square pixels.",
        sq.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect_can_hold = rect1.can_hold(&rect2);
    println!("rect1 can hold rect2: {}", rect_can_hold);
}