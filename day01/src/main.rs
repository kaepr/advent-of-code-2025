use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "input.txt";
    let contents = fs::read_to_string(path)?;

    println!("input: {}", contents);

    Ok(())
}
