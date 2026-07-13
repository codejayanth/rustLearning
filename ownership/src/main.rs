fn main() {
    let s1= String::from("Hello");
    let s2 = s1;
    println!("The content of s2 is {s2}"); // this works because ownership is moved to s2
    // println!("The content of s1 is {s1}"); // this doesn't work because ownership is already moved to s2, so s1 is dropped

    let s = String::from("Rama");
    obtain_ownership(&s); // giving the reference to the s rather than giving ownership, in this way we can use s later as well

    let value = 10;
    copy_trait(value);

    let my_name = String::from("Adam");
    let length = calculate_len(&my_name);
    //When functions have references as parameters instead of the actual values,
    //we won’t need to return the values in order to give back ownership, because we never had ownership.
    println!("The value of the length is: {length}");

    // what happens if we refer to something and change it?
    // let your_name = String::from("Steve");
    // say_full_name(&your_name);
    // println!("Your full name is: {full_name}"); // it won't work because the reference is immutable, it can only be referenced but not changeable.
    
    // we can do something to make it work which is making it mutable.
    let mut name = String::from("Steve");
    say_full_name_after_mutability(&mut name);


    //If you have a mutable reference to a value, you can have no other references to that value
    let mut a = String::from("Wassup");
    let b = &mut a;
    let c = &mut a; // as a side note also can't borrow the b. 
    println!("The value is {b}");


    // We also cannot have a mutable reference while we have an immutable one to the same value.
    let mut some_value = String::from("Data");
    let r1 = &some_value // works
    let r2 = &some_value // works
    let r3 = &mut r2 // doesn't work as we have an immutable one to the same value.


    // but in the code if r1 and r2 goes out of scope then r3 will work, for example:

    let mut some_value = String::from("Data");
    let r1 = &some_value // works
    let r2 = &some_value // works
    println!("The value of r1 and r2 is {r1} and {r2}");
    let r3 = &mut r2 // this works now, because r1 and r2 goes out of scope because of the println above. 

}

fn obtain_ownership(s: &String) {
    println!("Hey {s}"); 
}

fn copy_trait(some_value: i32) {
    println!("The value is {some_value}");
}

fn calculate_len(some_string: &String) -> usize {
    some_string.len()
}

// fn say_full_name(some_name: &String) {
//     some_name.push_str(", Jobs"); //  some_name.push_str(", Jobs");  `some_name` is a `&` reference, so it cannot be borrowed as mutable
// } 

fn say_full_name_after_mutability(some_name: &mut String){
    some_name.push_str(", Jobs")
}