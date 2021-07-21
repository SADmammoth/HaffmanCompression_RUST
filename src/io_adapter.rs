use std::fs::*;
use std::io::{self, Result, Write};

pub fn read() -> String {
    let path = get_path("Please, enter input file path:");
    read_to_string(path).expect("Something went wrong reading the file")
}

pub fn write(content: String) -> Result<()> {
    let path = get_path("Please, enter output file path:");
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn get_path(message: &str) -> String {
    println!("{}", message);
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    path.trim().to_string()
}
