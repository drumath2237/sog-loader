use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sog_file = fs::read("./crates/sample_data/pizza.sog")?;

    let unzipped = sog_decoder::decode::unzip(&sog_file)?;
    let sog_data = sog_decoder::decode::parse_sog(unzipped)?;

    println!("splat count:{}", sog_data.count);

    let splat = sog_decoder::decode::decode_sog(&sog_data)?;

    println!("count: {:?}", splat.count);

    Ok(())
}
