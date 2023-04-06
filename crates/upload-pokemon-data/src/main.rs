fn main() -> Result<(), csv::Error> {
    let path = "./crates/upload-pokemon-data/pokemon.csv";
    let mut rdr = csv::Reader::from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
