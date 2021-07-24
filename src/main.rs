mod haffman;
mod io_adapter;

fn main() {
    let message = io_adapter::read();
    let encoded = haffman::HaffmanCompression::new().compress(&message);
    // println!("{}", encoded.get_alphabet());
    // println!("{}", encoded.get_encoded());
    // println!("{}", encoded.get_with_injected_alphabet());
    println!("{}", encoded.analyze());

    io_adapter::write(encoded.get_with_injected_alphabet()).unwrap();

    let decoded =
        haffman::HaffmanDecompression::new().decompress(&encoded.get_with_injected_alphabet());
    // println!("{}", encoded.get_alphabet());
    // println!("{}", encoded.get_encoded());
    // println!("{}", encoded.get_with_injected_alphabet());
}
