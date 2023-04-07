use crate::pokemon_csv::PokemonCsv;
use inflector::Inflector;
use ksuid::Ksuid;
use sqlx::{Encode, MySql, Type};
use std::fmt;

#[derive(Debug, Clone)]
pub struct PokemonTableRow {
    pub id: PokemonId,
    pub slug: String,
    pub name: String,
    pub pokedex_id: u16,
    // pub abilities: Vec<String>,
    // pub typing: Vec<String>,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
    pub height: u16,
    pub weight: u16,
    pub generation: u16,
    pub female_rate: Option<f32>,
    pub genderless: bool,
    pub legendary_or_mythical: bool,
    pub is_default: bool,
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u16,
    // pub egg_groups: Vec<String>,
    pub base_happiness: u16,
    pub evolves_from: Option<String>,
    pub primary_color: String,
    pub number_pokemon_with_typing: f32,
    pub normal_attack_effectiveness: f32,
    pub fire_attack_effectiveness: f32,
    pub water_attack_effectiveness: f32,
    pub electric_attack_effectiveness: f32,
    pub grass_attack_effectiveness: f32,
    pub ice_attack_effectiveness: f32,
    pub fighting_attack_effectiveness: f32,
    pub poison_attack_effectiveness: f32,
    pub ground_attack_effectiveness: f32,
    pub fly_attack_effectiveness: f32,
    pub psychic_attack_effectiveness: f32,
    pub bug_attack_effectiveness: f32,
    pub rock_attack_effectiveness: f32,
    pub ghost_attack_effectiveness: f32,
    pub dragon_attack_effectiveness: f32,
    pub dark_attack_effectiveness: f32,
    pub steel_attack_effectiveness: f32,
    pub fairy_attack_effectiveness: f32,
}

#[derive(Clone, Copy)]
pub struct PokemonId(Ksuid);

impl PokemonId {
    pub fn new() -> Self {
        PokemonId(Ksuid::generate())
    }
}

impl fmt::Debug for PokemonId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PokemonId")
            .field(&self.0.to_base62())
            .finish()
    }
}

impl<'q> Encode<'q, MySql> for PokemonId {
    fn encode_by_ref(
        &self,
        buf: &mut <MySql as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let bytes: &[u8] = &self.0.to_base62().into_bytes();
        <&[u8] as Encode<MySql>>::encode(bytes, buf)
    }
}

impl Type<MySql> for PokemonId {
    fn type_info() -> <MySql as sqlx::Database>::TypeInfo {
        <&[u8] as Type<MySql>>::type_info()
    }

    fn compatible(ty: &<MySql as sqlx::Database>::TypeInfo) -> bool {
        <&[u8] as Type<MySql>>::compatible(ty)
    }
}

impl From<PokemonCsv> for PokemonTableRow {
    fn from(x: PokemonCsv) -> Self {
        PokemonTableRow {
            id: PokemonId::new(),
            slug: x.name.to_kebab_case(),
            name: x.name,
            pokedex_id: x.pokedex_id,
            hp: x.hp.into(),
            attack: x.attack.into(),
            defense: x.defense.into(),
            special_attack: x.special_attack.into(),
            special_defense: x.special_defense.into(),
            speed: x.speed.into(),
            height: x.height,
            weight: x.weight,
            generation: x.generation.into(),
            female_rate: x.female_rate,
            genderless: x.genderless,
            legendary_or_mythical: x.legendary_or_mythical,
            is_default: x.is_default,
            forms_switchable: x.forms_switchable,
            base_experience: x.base_experience,
            capture_rate: x.capture_rate.into(),
            base_happiness: x.base_happiness.into(),
            evolves_from: x.evolves_from,
            primary_color: x.primary_color,
            number_pokemon_with_typing: x.number_pokemon_with_typing,
            normal_attack_effectiveness: x.normal_attack_effectiveness,
            fire_attack_effectiveness: x.fire_attack_effectiveness,
            water_attack_effectiveness: x.water_attack_effectiveness,
            electric_attack_effectiveness: x.electric_attack_effectiveness,
            grass_attack_effectiveness: x.grass_attack_effectiveness,
            ice_attack_effectiveness: x.ice_attack_effectiveness,
            fighting_attack_effectiveness: x.fighting_attack_effectiveness,
            poison_attack_effectiveness: x.poison_attack_effectiveness,
            ground_attack_effectiveness: x.ground_attack_effectiveness,
            fly_attack_effectiveness: x.fly_attack_effectiveness,
            psychic_attack_effectiveness: x.psychic_attack_effectiveness,
            bug_attack_effectiveness: x.bug_attack_effectiveness,
            rock_attack_effectiveness: x.rock_attack_effectiveness,
            ghost_attack_effectiveness: x.ghost_attack_effectiveness,
            dragon_attack_effectiveness: x.dragon_attack_effectiveness,
            dark_attack_effectiveness: x.dark_attack_effectiveness,
            steel_attack_effectiveness: x.steel_attack_effectiveness,
            fairy_attack_effectiveness: x.fairy_attack_effectiveness,
        }
    }
}
