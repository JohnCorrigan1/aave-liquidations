use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("POOL", "abi/pool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;

    Ok(())
}
