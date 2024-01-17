#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u8,
    active: bool,
}

fn build_user(name: String, email: String, age: u8) -> User {
    // short hand way to instantiate a user
    let user = User {
        name,
        email,
        age,
        active: true,
    };
    // long form to instantiate a user
    // let user = User{
    //     name: name,
    //     email: email,
    //     age: age,
    //     active: true,
    // };
    user
}

fn main() {
    let mut user_one = build_user(
        String::from("Aayush Giri"),
        String::from("aayushgiri1234@gmail.com"),
        22_u8,
    );
    println!("{:?}", user_one);

    user_one.age = 32;

    println!("{:?}", user_one);

    let user_two = User {
        name: String::from("AD"),
        email: String::from("anksihdwidaan@gmail.com"),
        ..user_one
    };

    println!("{:?}", user_two);


    println!("Success");
}

/*
Attribute Usage: #[derive(Debug)] is a Rust attribute applied to structs or enums to automatically implement the Debug trait for them.

derive Keyword: It generates implementations of specified traits, in this case, the Debug trait, saving manual implementation effort.

Functionality of Debug Trait: Enables formatting of the struct or enum for debugging purposes, allowing instances to be printed using {:?} 
for easy inspection.
*/