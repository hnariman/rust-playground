use std::io;

mod arrays;
mod if_else;
mod loops;
mod strings;

fn main() {
    let value: i8 = 15;

    arrays::iterate();
    strings::iterate();

    if_else::classic(value.clone());
    if_else::rusty(value.clone());

    loops::while_loop(5);
    loops::just_loop(5);
    loops::labeled_loop(5);
    loops::for_loop(5);
    loops::for_array_iter(5);
    loops::reverse_for(10);
    // user_input();
}

fn user_input() {
    let mut response = String::new();
    println!("Shall we start?");
    io::stdin()
        .read_line(&mut response)
        .expect("failed to read user input");
    println!("user input is : {} ", response);
}
