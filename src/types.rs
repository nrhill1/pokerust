#[derive(Debug, Clone, Copy)]
mod pokemon;

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

fn calculate_multiplier(attack: &Move, defender: &Pokemon) -> f32 {
    let multiplier: f32 = 1.0;
    for t in &defender.types {
        match attack.move_type {
            Type::Electric => {
                // if defender.type 
            }
            Type::Water => {}
            Type::Fire => {}
            Type::Dark => {}
            Type::Grass => {}
            Type::Normal => {}
            Type::Flying => {}
            Type::Poison => {}
            Type::Bug => {}
            Type::Ground => {}
            Type::Rock => {}
            Type::Ghost => {}
            Type::Steel => {}
            Type::Ice => {}
            Type::Dragon => {}
            Type::Psychic => {}
            Type::Fighting => {}
            Type::Fairy => {}
        }
    }
    multiplier
}