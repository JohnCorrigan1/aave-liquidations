use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC721", "abi/erc721.json")?
        .generate()?
        .write_to_file("src/abi/erc721.rs")?;

    Abigen::new("POOL", "abi/pool.json")?
        .generate()?
        .write_to_file("src/abi/pool.rs")?;

    Ok(())
}
