
pub struct PokemonTableRow {
    pub id: Ksuid,
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
    // pub genderless: bool,
    pub is_legendary_or_mythical: bool,
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
