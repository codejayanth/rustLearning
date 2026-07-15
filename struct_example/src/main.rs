#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("The content of rect1 is: {:?}", rect1); // {:?} is to print the fields of struct, because `Rectangle` implement `Debug`
    println!("The content of rect1 is: {:#?}", rect1); // in multiline

    dbg!(&rect1); // even simpler, the same can be used for the fields inside the struct
    let area_of_rect1 = area(&rect1);
    println!("The are of rect1 is: {area_of_rect1}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}