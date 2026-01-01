use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", sog_decoder::add(1, 2));

    println!("{:?}", env::current_dir()?);
    let sog_file = fs::read("./crates/sample_data/pizza.sog")?;

    let data = sog_decoder::extract_zip(&sog_file)?;

    Ok(())
}
