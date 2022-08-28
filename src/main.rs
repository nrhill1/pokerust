#[derive(Debug)]
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
    pub move_type: Type,
}

impl Move {
    pub fn new(name: String, damage: u32, accuracy: u32, move_type: Type) -> Self {
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
    pub types: Vec<Type>,
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
        types: Vec<Type>,
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

#[derive(Debug)]
struct Trainer {
    name: &'static str,
    team: Vec<Pokemon>,
    money: i64,
}

impl Trainer {
    pub fn new(name: &'static str, team: Vec<Pokemon>, money: i64) -> Self {
        Self { name, team, money }
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
        vec![Type::Psychic],
        vec![
            Move::new("Psybeam".to_string(), 65, 100, Type::Psychic),
            Move::new("Psychic".to_string(), 90, 100, Type::Psychic),
            Move::new("Psyshock".to_string(), 80, 100, Type::Psychic),
            Move::new("Future Sight".to_string(), 120, 100, Type::Psychic),
        ],
    );

    let charizard = Pokemon::new(
        "Charizard".to_string(),
        50,
        78,
        84,
        78,
        100,
        109,
        85,
        vec![Type::Fire, Type::Flying],
        vec![
            Move::new("Flamethrower".to_string(), 90, 100, Type::Fire),
            Move::new("Fire Blast".to_string(), 110, 85, Type::Fire),
            Move::new("Air Slash".to_string(), 75, 95, Type::Flying),
            Move::new("Dragon Claw".to_string(), 80, 100, Type::Dragon),
        ],
    );

    println!("{:#?}", espeon);
    println!("{:#?}", charizard);
}
