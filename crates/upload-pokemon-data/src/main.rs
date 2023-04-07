mod pokemon_csv;
use color_eyre::{
    eyre::{self, Context},
    Help,
};
use pokemon_csv::*;
mod db;
use db::*;
use indicatif::ProgressIterator;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let database_url = env::var("DATABASE_URL")
        .wrap_err("Must have a DATABASE_URL set")
        .suggestion("Un `pscale connect <database> <branch>` to get an connection")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .suggestion(
            "database url must be in the form `mysql://username:password@host:port/database",
        )?;
    let path = "./crates/upload-pokemon-data/pokemon.csv";
    let mut rdr = csv::Reader::from_path(path)?;
    let pokemon = rdr
        .deserialize()
        .collect::<Result<Vec<PokemonCsv>, csv::Error>>()?;

    for record in pokemon.into_iter().progress() {
        let pokemon_row: PokemonTableRow = record.into();
        insert_pokemon(&pool, &pokemon_row).await?;
    }

    Ok(())
}

pub async fn insert_pokemon(
    pool: &MySqlPool,
    x: &PokemonTableRow,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
    insert into pokemon (
        id,
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness
     ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"#,
        x.id,
        x.slug,
        x.name,
        x.pokedex_id,
        x.hp,
        x.attack,
        x.defense,
        x.special_attack,
        x.special_defense,
        x.speed,
        x.height,
        x.weight,
        x.generation,
        x.female_rate,
        x.genderless,
        x.legendary_or_mythical,
        x.is_default,
        x.forms_switchable,
        x.base_experience,
        x.capture_rate,
        x.base_happiness,
        x.primary_color,
        x.number_pokemon_with_typing,
        x.normal_attack_effectiveness,
        x.fire_attack_effectiveness,
        x.water_attack_effectiveness,
        x.electric_attack_effectiveness,
        x.grass_attack_effectiveness,
        x.ice_attack_effectiveness,
        x.fighting_attack_effectiveness,
        x.poison_attack_effectiveness,
        x.ground_attack_effectiveness,
        x.fly_attack_effectiveness,
        x.psychic_attack_effectiveness,
        x.bug_attack_effectiveness,
        x.rock_attack_effectiveness,
        x.ghost_attack_effectiveness,
        x.dragon_attack_effectiveness,
        x.dark_attack_effectiveness,
        x.steel_attack_effectiveness,
        x.fairy_attack_effectiveness,
    )
    .execute(pool)
    .await
}
