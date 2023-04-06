mod pokemon_csv;
use pokemon_csv::*;

fn main() -> Result<(), csv::Error> {
    let path = "./crates/upload-pokemon-data/pokemon.csv";
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.deserialize() {
        let record: PokemonCsv = result?;
        println!("{:?}", record);
    }

    Ok(())
}
