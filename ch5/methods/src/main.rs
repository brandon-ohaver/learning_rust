struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect3 = Rectangle {
        width:30,
        height: 55,
    };

    println!("The area of rect1 is {}", rect1.area());
    println!("Does rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Does rect1 hold rect3? {}", rect1.can_hold(&rect3));

}
