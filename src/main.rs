struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("The area of rect1 is {}", rect1.area());
    println!("The area of rect2 is {}", rect2.area());
    println!("The area of rect3 is {}", rect3.area());

    let rect4 = Rectangle::square(20, 60);
    println!("The area of rect4 is {}", rect4.area());
}
