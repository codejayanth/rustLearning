// There are three kinds of loops in rust:
// loop, for, while

fn main() {
    // loop 
    // loop {
    //     println!("Rusttt");
    // } // this runs infinitely until stopped.
    let mut counter = 0;
    loop {
        println!("Looping {counter} time");
        counter += 1;
        if counter == 10 {
            break
        }
    } // This loops for 10 times and breaks

    // while loop 
    let mut count = 0;
    while count < 5 {
        println!("Printing something");
        count += 1;
    }

    // for loop
    let brand_names = ["Tesla", "Ford", "Audi"];
    for index in brand_names {
        println!("The car brand is: {index}");
    }

    let a = [10, 20, 30, 40, 50];
    for item in a {
        println!("{item}");
    }

    for i in (0..7).rev() {
        println!("{i}");
    }

}


