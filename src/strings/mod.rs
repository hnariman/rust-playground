mod chars;
mod chars2;

pub fn iterate() {
    let message = "this is some unicode-8 string!".to_string();
    println!("string iteration here");

    chars::iterate(message.clone());
    chars2::iterate(message.clone());
}
