mod haffman;
use std::io;

fn main() {
    println!("Please, enter message:");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");

    let query = haffman::create_priority_queue(message.trim());
    println!("{}", query);
    let tree = haffman::create_tree(query);
    println!("{}", tree);
    let alphabet = haffman::create_alphabet(tree);
    println!("{:?}", alphabet);
    let encoded = haffman::compress(&message, alphabet);
    println!("{}", encoded);
}
