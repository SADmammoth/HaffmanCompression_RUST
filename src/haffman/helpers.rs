pub fn pad(string: &str, len: usize, symbol: char) -> String {
    if len <= string.len() {
        string.to_string()
    } else {
        let spaces = std::iter::repeat(symbol)
            .take(len - string.len())
            .collect::<String>();
        format!("{}{}", spaces, string)
    }
}

pub fn char_to_bin(character: &char) -> String {
    let mut char_number = [0u8; 1];
    character.encode_utf8(&mut char_number);

    let mut string = String::new();
    for symbol in char_number.iter() {
        string += &format!("{:b}", symbol);
    }

    string
}

pub fn text_to_bin(text: &str) -> String {
    text.bytes()
        .fold(String::new(), |string, x| string + &format!("{:b}", x))
}

pub fn num_to_bin(num: usize) -> String {
    format!("{:b}", num)
}
