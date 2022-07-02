#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!("rect is {:?}", rect);

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

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

    println!("CAn rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("CAn rect1 hold rect3? {}", rect1.can_hold(&rect3));

    dbg!(Rectangle::square(3));
}
