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
    println!("Hello, world!");
}
