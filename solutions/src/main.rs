use std::fs;
use utils::hello_world;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("../inputs/day0.txt")?;
    let name = input.trim();
    println!("Hello, {}!", name);
    println!("{}", hello_world());
    Ok(())
}
