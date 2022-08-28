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

    let pikachu = Pokemon::new(
        "Pikachu".to_string(),
        50,
        35,
        55,
        40,
        90,
        50,
        50,
        vec![Type::Electric],
        vec![
            Move::new("Thunderbolt".to_string(), 90, 100, Type::Electric),
            Move::new("Thunder".to_string(), 110, 70, Type::Electric),
            Move::new("Thunder Punch".to_string(), 75, 100, Type::Electric),
            Move::new("Volt Tackle".to_string(), 120, 100, Type::Electric),
        ],
    );

    let totodile = Pokemon::new(
        "Totodile".to_string(),
        50,
        50,
        65,
        64,
        43,
        44,
        48,
        vec![Type::Water],
        vec![
            Move::new("Waterfall".to_string(), 80, 100, Type::Water),
            Move::new("Hydro Pump".to_string(), 110, 80, Type::Water),
            Move::new("Ice Fang".to_string(), 65, 95, Type::Ice),
            Move::new("Crunch".to_string(), 80, 100, Type::Dark),
        ],
    );

    let breloom = Pokemon::new(
        "Breloom".to_string(),
        50,
        60,
        130,
        80,
        70,
        60,
        60,
        vec![Type::Grass, Type::Fighting],
        vec![
            Move::new("Mach Punch".to_string(), 40, 100, Type::Fighting),
            Move::new("Dynamic Punch".to_string(), 100, 50, Type::Fighting),
            Move::new("Bullet Seed".to_string(), 25, 100, Type::Grass),
            Move::new("Focus Blast".to_string(), 120, 70, Type::Fighting),
        ],
    );

    let snorlax = Pokemon::new(
        "Snorlax".to_string(),
        50,
        160,
        110,
        65,
        30,
        65,
        110,
        vec![Type::Normal],
        vec![
            Move::new("Body Slam".to_string(), 85, 100, Type::Normal),
            Move::new("Earthquake".to_string(), 100, 100, Type::Ground),
            Move::new("Crunch".to_string(), 80, 100, Type::Dark),
            Move::new("Giga Impact".to_string(), 150, 90, Type::Normal),
        ],
    );

    let ash = Trainer::new(
        "Ash",
        vec![espeon, charizard, pikachu, totodile, breloom, snorlax],
        1000,
    );
}
