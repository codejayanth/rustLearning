fn main() {
    let x = 5; // int
    println!("The value of x is: {x}");

    let name = "Ram"; // String
    println!("The name is: {name}");

    // x = 10;
    // println!("The value of x is: {x}"); // This will give you compile error, because the x here is not mutable.

    // let's create a mutable variable y
    let mut y = 9;
    println!("The value of y is: {y}");

    y = 10;
    println!("The new value of y is: {y}");

    // let's declar some constants
    const PI: f64=3.14;
    const TWO_HOURS_IN_SECONDS: u32 = 60*60*3;

    println!("The two constants here are: {PI}, {TWO_HOURS_IN_SECONDS}");

    // let's look at the shadowing concept
    let z = 10;
    let z = z + 1; // shadowing
    {
        let z = z * 3;
        println!("The value of z is: {z}"); // It's 33
    }
    println!("The value of z is: {z}"); // It is 11


    // Some of the other integers are: // scalar types btw
    let addition = 10 + 5;
    let substraction = 95.5 - 10.22;
    let multiplication = 10*3;
    let quotient = 34.1/45.2;
    let remainder = 34 % 5; // remainder

    let is_cricket_great = true; // boolean
    let love = '😍'; // char here
    println!("With love: {love} rust!");

    // rust has two primitive compound types
    // tuples
    let students_list: (String, i16, f64) = (String::from("Rama"), 12, 90.6);
    let name_of_the_student = students_list.0; // index starts from 0
    let id_of_student = students_list.1; 
    let percentage_of_student = students_list.2;

    println!("The name is: {name_of_the_student} and his id is: {id_of_student} and percentage is: {percentage_of_student}");
 
    // arrays
    // arrays should have the same type unlike tuple

    let cars_list = ["BMW", "Tesla", "Audi"];
    println!("The first car in the array is: {}", cars_list[0]);

    let months_in_year: [i32; 12] = [1,2,3,4,5,6,7,8,9,10,11,12]; // another way of writing an array

    let c = [4; 5]; // 4 is repeated 5 times.

 
 }