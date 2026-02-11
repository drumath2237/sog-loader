use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read("./crates/sample_data/pizza.sog")?;

    println!("start decoding");

    let time = std::time::Instant::now();
    let sog = sog_decoder::unpack(&file)?;
    let splat = sog_decoder::decode(&sog)?;
    let elapsed = time.elapsed();

    println!("count: {:?}", splat.count);
    println!("elapsed: {:?}", elapsed);
    println!("done");

    Ok(())
}
