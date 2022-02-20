mod chars;
mod chars2;

pub fn iterate() {
    let message = "unicode-8".to_string();
    println!("iteration");

    chars::iterate(message.clone());
    chars2::iterate(message.clone());
}
