pub struct Move {
    pub name: String,
    pub damage: u32,
    pub accuracy: u32,
    pub move_type: String,
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


fn main() {
    println!("Hello, world!");
}
