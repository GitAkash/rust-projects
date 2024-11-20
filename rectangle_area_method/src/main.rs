struct Rectangle {
    width: u32,
    height: u32,
}

// Everything associated in this 'implementation'
// will be associated with the rectangle.
// This will help with organization

impl Rectangle {
    // pass itself into the associated function
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    // Function with another instance
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an associated function, more particular it is a constructor.
    // It doesn't have self as their first parmeter.
    // Because they don't need an instance of the type to work with.
    // You "construct" a new rectangle with certain attributes.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Creating rectangles with their values
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

    // Using the constructor to create a square.
    let sq = Rectangle::square(3);

    // Output
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("The area of the square is {} square pixels.", sq.area());
}
