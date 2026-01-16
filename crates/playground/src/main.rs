use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read("./crates/sample_data/pizza.sog")?;
    let sog = sog_decoder::unpack(&file)?;
    let splat = sog_decoder::decode(&sog)?;
    println!("count: {:?}", splat.count);
    Ok(())
}
