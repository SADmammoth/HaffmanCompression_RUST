use std::fs::*;
use std::io::{self, Result, Write};
pub fn read() -> String { 
	println!("Please, enter file path:");
	let mut path = String::new();
	io::stdin()
	.read_line(&mut path)
	.expect("Failed to read line");

	read_to_string(path.trim())
	.expect("Something went wrong reading the file")
}

pub fn write(content: String) -> Result<()> {
	println!("Please, enter output file path:");
	let mut path = String::new();
	io::stdin()
	.read_line(&mut path)
	.expect("Failed to read line");

	let mut file = File::create(path.trim())?;
    file.write_all(content.as_bytes())?;
	Ok(())
}