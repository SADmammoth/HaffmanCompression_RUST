mod haffman;
use haffman::{compress, init};
use std::io;

fn main() {
    println!("Please, enter message:");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    let query = init::create_priority_queue(message.trim());
    println!("{}", query);
    let tree = init::create_tree(query);
    println!("{}", tree);
    let alphabet = init::create_alphabet(tree);
    println!("{:?}", alphabet);
    let encoded = compress::compress(&message, &alphabet);
    println!("{}", encoded);
    println!("{}", alphabet.encode_info() + &encoded);
}
