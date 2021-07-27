mod haffman;
mod io_adapter;

fn main() {
    let message = io_adapter::read();
    let encoded = haffman::HaffmanCompression::new().compress(&message);

    println!("{}", encoded.analyze());

    io_adapter::write(encoded.get_with_injected_alphabet()).unwrap();

    let decoded =
        haffman::HaffmanDecompression::new().decompress(&encoded.get_with_injected_alphabet());

    println!("{}", decoded.analyze());

    io_adapter::write(decoded.get_decoded()).unwrap();
}
