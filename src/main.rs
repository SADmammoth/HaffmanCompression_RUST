mod haffman;
use std::io;

fn main() {
    println!("Please, enter message:");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    let message = message.trim();

    let encoded = haffman::HaffmanCompression::new().compress(&message);
    println!("{}", encoded.get_alphabet());
    println!("{}", encoded.get_encoded());
    println!("{}", encoded.get_with_injected_alphabet());
}
