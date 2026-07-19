// let's write some functions and structs and methods 
// let's create the customer struct
#[derive(Debug)]
struct Customer {
    name: String,
    cust_id: u32,
    phonenumber: u32,
}


fn main() {
    let customer1 = Customer {
        name: String::from("Rama"),
        cust_id: 1,
        phonenumber: 53465787,
    };

    println!("The customer1 name is: {}", customer1.name);

    let greet = say_hi_to_customer(&customer1.name);
    println!("{}", greet);
}

fn say_hi_to_customer(customer_name: &String) -> &str  {
    "hi"
}
