use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sog_file = fs::read("./crates/sample_data/pizza.sog")?;
    let splat = sog_decoder::decode(&sog_file)?;
    println!("count: {:?}", splat.count);
    Ok(())
}
