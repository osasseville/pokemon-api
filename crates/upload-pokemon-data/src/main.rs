mod pokemon_csv;
use pokemon_csv::*;
mod db;
use db::*;

fn main() -> Result<(), csv::Error> {
    let path = "./crates/upload-pokemon-data/pokemon.csv";
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: PokemonCsv = result?;
        let pokemon_row : PokemonTableRow = record.into();
        println!("{:?}", pokemon_row);
    }
    
    dbg!(PokemonId::new());

    Ok(())
}
