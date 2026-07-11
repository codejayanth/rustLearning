fn main() {
    let number = 45;
    if number > 10 && number < 50 {
        println!("The number is between 10 and 50");
    } else if number < 10 {
        println!("The number is below 10");
    } else {
        println!("The number is above 50");
    }

    // using if in a let statement
    let condition = true;
    let number = if condition { 3 } else { 4 };
    println!("The number is {number}");

    // more if-else
    let number = 60;
    if number % 2 == 0 {
        println!("The number is divisble by 2");
    } else if number % 4 == 0 {
        println!("The number is divisble by 4");
    } else if number % 5 == 0 {
        println!("The number is divisible by 5");
    } else if number % 7 == 0 {
        println!("The number is divisible by 7");
    } else {
        println!("Give the proper number");
    }
    // we see number divisible by 2, we don't see other print statements
    // that's because rust only executes the block for the first true condition
    // once it finds one, it doesn't even check the rest.
}


