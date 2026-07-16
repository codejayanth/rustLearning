#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn height(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
}

fn main () {
    let rect1 = Rectangle {
        width: 23,
        height: 43,
    };

    println!("The area of the rect1 is: {:?}", rect1.area());

    println!("Is the width available in the rect1? {:?}", rect1.height());

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect2 hold rect3?: {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect3?: {}", rect3.can_hold(&rect4));


}