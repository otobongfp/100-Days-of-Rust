#[derive(Debug)] //Calling Debug explicitly makes it possible to format a struct through the println! macro

// struct Rectangle {
//     width : u32,
//     height : u32,
// }

// fn main() {

//     let rect = Rectangle{
//         width : 40,
//         height : 10,
//     };

//     println!("The area of the rectangle is {}",area(&rect));
//     println!("The rect struct can be printed as {:#?}",rect);
//     dbg!(&rect);
// }

// fn area(rectangle : &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
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


    println!("Print-Out the height is {} & the width is {}", rect1.height, rect1.width);
    println!("The area is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}