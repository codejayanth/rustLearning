fn main() {
    // creating the variable and assigning the value

    let num = 5;
    println!("The value of num is: {num}"); // printing the number
    // num = 6; // assigning a new value
    // println!("The value of num is: {num}"); 

    // the above will assignment will fail because num is not mutable, so we creat the mutable variable
    let mut num_two = 5;
    println!("The  value of num_two is: {num_two}");
    num_two = 6;
    println!("The  new value of num_two is: {num_two}");

    let mut name = String::from("Rama");
    name.push_str(" jayanth");
    println!("The name is: {name}");

    // loop 
    let mut value = 5;
    loop {
        println!("Hey hii");
        if value == 0 {
            break;
        }
        value -= 1;
    }

    // while loop
    let mut new_value = 5;
    while new_value > 0 {
        println!("Have a nice day");
        new_value -= 1;
    }

    // for loop
    for _ in 0..6 {
        println!("Hello wassup");
    }

    // iterating an array
    let car_brands = ["ford", "tesla", "audi"];
    for i in car_brands {
        println!("{}", i);
    }

    // functions 
    let a = 10;
    let b = 20;
    println!("The sum of value a and b is: {}", sum(&a, &b));

}

fn sum(a: &u32, b: &u32) -> u32 {
    a + b
}