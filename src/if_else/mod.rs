pub fn classic(value: i8) {
    if value == 5 {
        println!("its not a small number!");
    } else if value > 10 {
        println!("it even more than 10!");
    } else {
        println!("We need some REALLY BIG number")
    }
}

// Rust has powerful brother of ternary operator
// but mind the strict typing, return only same type
// better memory, but readability isn't yet there for me

pub fn rusty(value: i8) {
    let result: &str = if value == 5 {
        "its not a small number"
    } else if value > 10 {
        "its even bigger than 10"
    } else {
        "we need some real number though"
    };
    println!("{}", result);
}
