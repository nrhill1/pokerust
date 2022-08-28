pub enum Type {
    Electric,
    Water,
    Fire,
    Dark,
    Grass,
    Normal,
    Flying,
    Poison,
    Bug,
    Ground,
    Rock,
    Ghost,
    Steel,
    Ice,
    Dragon,
    Psychic,
    Fighting,
    Fairy,
}



#[derive(Debug)]
pub struct Move {
    pub name: String,
    pub damage: u32,
    pub accuracy: u32,
    pub move_type: String,
}

impl Move {
    pub fn new(name: String, damage: u32, accuracy: u32, move_type: String) -> Self {
        Self {
            name,
            damage,
            accuracy,
            move_type,
        }
    }
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub level: u8,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub speed: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub types: Vec<String>,
    pub moves: Vec<Move>,
}

impl Pokemon {
    pub fn new(
        name: String,
        level: u8,
        hp: u8,
        attack: u8,
        defense: u8,
        speed: u8,
        special_attack: u8,
        special_defense: u8,
        types: Vec<String>,
        moves: Vec<Move>,
    ) -> Self {
        Self {
            name,
            level,
            hp,
            attack,
            defense,
            speed,
            special_attack,
            special_defense,
            types,
            moves,
        }
    }
}

fn main() {
    let espeon = Pokemon::new(
        "Espeon".to_string(),
        50,
        65,
        65,
        60,
        110,
        130,
        95,
        vec!["Psychic".to_string()],
        vec![
            Move::new("Psybeam".to_string(), 65, 100, "Psychic".to_string()),
            Move::new("Psychic".to_string(), 90, 100, "Psychic".to_string()),
            Move::new("Psyshock".to_string(), 80, 100, "Psychic".to_string()),
            Move::new("Future Sight".to_string(), 120, 100, "Psychic".to_string()),
        ],
    );

    println!("{:#?}", espeon);
}
