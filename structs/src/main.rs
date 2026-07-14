struct Student {
    name: String,
    id: u32,
    percentage: f64,
} // simple representation of the struct

struct Rectangle {
    width: u32,
    height: u32,
}

struct User {
    name: String,
    email: String,
    is_active: bool,
    sign_in_count: u32,
}

// tuple stucts
struct Color(i32, i32, i32);
struct Origin(i32, i32, i32);

fn main() {
    let student_one = Student {
        name: String::from("Ram"),
        id: 32,
        percentage: 83.3,
    };

    println!("The name of the student_one is: {} and id is: {} and percentage is: {}", student_one.name, student_one.id, student_one.percentage);

    let rect1 = Rectangle {
        width: 32,
        height: 42,
    }; 

    let area_of_rect = area(&rect1); // referencing to the rect1 struct 
    println!("The area of the Rectangle is: {area_of_rect}");


    // creating the instance of the User
    let user1 = User {
        is_active: true,
        sign_in_count: 32,
        name: String::from("Jayanth"),
        email: String::from("ram@gmail.com"),
    };

    // if we want to update the created instance
    let mut user2 = User {
        is_active: true,
        sign_in_count: 43,
        name: String::from("Jayanth"),
        email: String::from("ram@gmail.com"),
    };

    user2.name = String::from("jay");
    user2.email = String::from("rammm@gmail.com");

    // if we want to make the user3 instance by taking few values of the user2 instance
    let user3 = User {
        sign_in_count: 34,
        is_active: true,
        ..user2
    }; // then the user2's name and email can't be accessed anymore

    println!("The user2's name is: {}", user2.sign_in_count);// this is accessible because we haven't took them from user2

    // tuple structs
    let organge = Color(0, 5, 6);


}

fn area(some_struct: &Rectangle) -> u32 {
    some_struct.width * some_struct.height
}