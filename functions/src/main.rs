fn main() {
    // calling the function wassup 
    print_wassup(); // the main function is where the program runs.
    let sum = add_values(12, 34);
    println!("Calculated sum is: {sum}");

    let greeting = say_how_you_doing("Rama");
    println!("{greeting}");

}

// let's create the first function which just prints wassup
fn print_wassup() {
    println!("Hey, wassup");
}

// let's create the second function which takes two parameters and returns the sum as result
fn add_values(a: i16, b: i16) -> i16 {
    a+b
}

// let's do one more function which takes string as an input and says how are you doing followed by name
fn say_how_you_doing(name: &str) -> String {
    format!("Hey how are you doing {name}")
}