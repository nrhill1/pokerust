#![allow(dead_code, unused_mut)]

#[derive(Debug, Clone, Copy)]
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
    let mut multiplier: f32 = 1.0;
    let att_type = attack.move_type;
    for t in defender.types.clone() {
        match att_type {
            Type::Electric => match t {
                Type::Ground => multiplier * 0.0,
                Type::Flying => multiplier * 2.0,
                Type::Water => multiplier * 2.0,
                Type::Grass => multiplier * 0.5,
                Type::Electric => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Water => match t {
                Type::Fire => multiplier * 2.0,
                Type::Ground => multiplier * 2.0,
                Type::Rock => multiplier * 2.0,
                Type::Water => multiplier * 0.5,
                Type::Grass => multiplier * 0.5,
                Type::Dragon => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Fire => match t {
                Type::Fire => multiplier * 0.5,
                Type::Water => multiplier * 0.5,
                Type::Grass => multiplier * 2.0,
                Type::Ice => multiplier * 2.0,
                Type::Bug => multiplier * 2.0,
                Type::Rock => multiplier * 0.5,
                Type::Dragon => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Dark => match t {
                Type::Fighting => multiplier * 2.0,
                Type::Dark => multiplier * 0.5,
                Type::Ghost => multiplier * 0.5,
                Type::Psychic => multiplier * 0.0,
                _ => multiplier,
            },
            Type::Grass => match t {
                Type::Water => multiplier * 2.0,
                Type::Ground => multiplier * 2.0,
                Type::Rock => multiplier * 2.0,
                Type::Fire => multiplier * 0.5,
                Type::Grass => multiplier * 0.5,
                Type::Poison => multiplier * 0.5,
                Type::Flying => multiplier * 0.5,
                Type::Bug => multiplier * 0.5,
                Type::Dragon => multiplier * 0.5,
                Type::Steel => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Normal => match t {
                Type::Rock => multiplier * 0.5,
                Type::Steel => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Flying => match t {
                Type::Electric => multiplier * 0.5,
                Type::Rock => multiplier * 2.0,
                Type::Steel => multiplier * 0.5,
                Type::Grass => multiplier * 2.0,
                Type::Fighting => multiplier * 2.0,
                Type::Bug => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Poison => match t {
                Type::Grass => multiplier * 2.0,
                Type::Fairy => multiplier * 2.0,
                Type::Fighting => multiplier * 0.5,
                Type::Poison => multiplier * 0.5,
                Type::Bug => multiplier * 0.5,
                Type::Psychic => multiplier * 0.5,
                Type::Ghost => multiplier * 0.5,
                Type::Steel => multiplier * 0.0,
                _ => multiplier,
            },
            Type::Bug => match t {
                Type::Grass => multiplier * 2.0,
                Type::Fighting => multiplier * 0.5,
                Type::Poison => multiplier * 0.5,
                Type::Flying => multiplier * 2.0,
                Type::Ghost => multiplier * 0.5,
                Type::Steel => multiplier * 0.5,
                Type::Fire => multiplier * 0.5,
                Type::Psychic => multiplier * 2.0,
                Type::Dark => multiplier * 2.0,
                _ => multiplier,
            },
            Type::Ground => match t {
                Type::Poison => multiplier * 0.5,
                Type::Rock => multiplier * 0.5,
                Type::Steel => multiplier * 2.0,
                Type::Fire => multiplier * 2.0,
                Type::Electric => multiplier * 2.0,
                Type::Grass => multiplier * 0.5,
                Type::Flying => multiplier * 0.0,
                _ => multiplier,
            },
            Type::Rock => match t {
                Type::Normal => multiplier * 0.5,
                Type::Flying => multiplier * 0.5,
                Type::Poison => multiplier * 0.5,
                Type::Fire => multiplier * 2.0,
                Type::Ice => multiplier * 2.0,
                Type::Fighting => multiplier * 2.0,
                Type::Ground => multiplier * 2.0,
                Type::Steel => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Ghost => match t {
                Type::Normal => multiplier * 0.0,
                Type::Ghost => multiplier * 2.0,
                Type::Psychic => multiplier * 2.0,
                Type::Dark => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Steel => match t {
                Type::Normal => multiplier * 0.5,
                Type::Flying => multiplier * 2.0,
                Type::Poison => multiplier * 0.0,
                Type::Rock => multiplier * 2.0,
                Type::Steel => multiplier * 0.5,
                Type::Fire => multiplier * 0.5,
                Type::Water => multiplier * 0.5,
                Type::Electric => multiplier * 0.5,
                Type::Ice => multiplier * 2.0,
                Type::Bug => multiplier * 2.0,
                Type::Dragon => multiplier * 2.0,
                Type::Fairy => multiplier * 2.0,
                _ => multiplier,
            },
            Type::Ice => match t {
                Type::Ice => multiplier * 0.5,
                Type::Steel => multiplier * 0.5,
                Type::Fire => multiplier * 0.5,
                Type::Fighting => multiplier * 2.0,
                Type::Rock => multiplier * 2.0,
                _ => multiplier,
            },
            Type::Dragon => match t {
                Type::Dragon => multiplier * 2.0,
                Type::Steel => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Psychic => match t {
                Type::Fighting => multiplier * 2.0,
                Type::Psychic => multiplier * 0.5,
                Type::Dark => multiplier * 0.0,
                _ => multiplier,
            },
            Type::Fighting => match t {
                Type::Normal => multiplier * 2.0,
                Type::Flying => multiplier * 0.5,
                Type::Poison => multiplier * 0.5,
                Type::Rock => multiplier * 0.5,
                Type::Bug => multiplier * 0.5,
                Type::Ghost => multiplier * 0.0,
                Type::Steel => multiplier * 2.0,
                Type::Psychic => multiplier * 0.5,
                Type::Ice => multiplier * 2.0,
                Type::Dark => multiplier * 2.0,
                Type::Fairy => multiplier * 0.5,
                _ => multiplier,
            },
            Type::Fairy => match t {
                Type::Fighting => multiplier * 2.0,
                Type::Poison => multiplier * 2.0,
                Type::Bug => multiplier * 0.5,
                Type::Steel => multiplier * 0.5,
                Type::Fire => multiplier * 0.5,
                Type::Dark => multiplier * 2.0,
                Type::Dragon => multiplier * 0.0,
                _ => multiplier,
            },
        };
    }
    multiplier
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    name: String,
    team: Vec<Pokemon>,
    money: u64,
}

impl Trainer {
    pub fn new(name: String, team: Vec<Pokemon>, money: u64) -> Self {
        let mut _team = Vec::with_capacity(6);
        for pokemon in team {
            _team.push(pokemon);
        }
        Self {
            name,
            team: _team,
            money,
        }
    }

    pub fn add_pokemon(&mut self, pokemon: Pokemon) {
        self.team.push(pokemon);
    }
}

struct PC<'a> {
    box_count: u8,
    boxes: Vec<PcBox>,
    owner: &'a mut Trainer,
}

struct PcBox {
    pokemon: Vec<Pokemon>,
}

impl PcBox {
    pub fn new(pokemon: Vec<Pokemon>) -> Self {
        Self { pokemon }
    }
}

impl<'a> PC<'a> {
    pub fn new (owner: &'a mut Trainer) -> Self {
        Self {
            box_count: 1,
            boxes: vec![PcBox {
                pokemon: Vec::with_capacity(30),
            }],
            owner,
        }
    }

    pub fn deposit(&mut self) {
        if self.boxes[self.box_count as usize - 1].pokemon.len() == 30 {
            self.box_count += 1;
            self.boxes.push(PcBox {
                pokemon: Vec::with_capacity(30),
            });
        }
        self.boxes[self.box_count as usize - 1].pokemon.push(self.owner.team.pop().unwrap());
}

    pub fn withdraw(&mut self, index: usize) {
        let pokemon = self.boxes[self.box_count as usize - 1].pokemon.remove(index);
        self.owner.team.push(pokemon);
    }

    pub fn switch_box(&mut self, index: usize) {
        self.box_count = index as u8;
    }

    pub fn get_box(&self) -> &PcBox {
        &self.boxes[self.box_count as usize - 1]
    }

}

#[derive(Debug)]
struct Battle<'a> {
    player: &'a Trainer,
    opponent: &'a Trainer,
}

impl<'a> Battle<'a> {

    pub fn new(player: &Trainer, opponent: &Trainer) -> Self {
        Self { player, opponent }
    }

    pub fn start(&mut self) {
        println!("\n{} vs {}\n", self.player.name, self.opponent.name);
        println!("{}'s team:", self.player.name);
        for pokemon in &self.player.team {
            println!("{} - {}", pokemon.name, pokemon.level);
        }
        println!("\n{}'s team:", self.opponent.name);
        for pokemon in &self.opponent.team {
            println!("{} - {}", pokemon.name, pokemon.level);
        }
    }

    pub fn attack(&mut self, attacker: &mut Pokemon, defender: &mut Pokemon, move_index: usize) {
        let move_ = &attacker.moves[move_index];
        let damage = calculate_multiplier(move_, &defender);
        defender.hp -= damage as u8;
        println!(
            "{} used {} on {} for {} damage!",
            attacker.name, move_.name, defender.name, damage
        );
    }

    pub fn battle(&mut self) {
        self.start();
        let mut player_pokemon = &mut self.player.team[0];
        let mut opponent_pokemon = &mut self.opponent.team[0];
        println!("{} sent out {}!", self.player.name, player_pokemon.name);
        println!("{} sent out {}!", self.opponent.name, opponent_pokemon.name);
        // loop {
        //     self.attack(player_pokemon, opponent_pokemon, 0);
        //     if opponent_pokemon.hp == 0 {
        //         println!("{} fainted!", opponent_pokemon.name);
        //         break;
        //     }
        //     self.attack(opponent_pokemon, player_pokemon, 0);
        //     if player_pokemon.hp == 0 {
        //         println!("{} fainted!", player_pokemon.name);
        //         break;
        //     }
        // }
    }
}

pub fn main() {

    // Ash's Team
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



    // Nic's team
    let alolan_raichu = Pokemon::new(
        "Raichu-Alolan".to_string(),
        50,
        60,
        85,
        50,
        110,
        95,
        85,
        vec![Type::Electric, Type::Psychic],
        vec![
            Move::new("Thunderbolt".to_string(), 90, 100, Type::Electric),
            Move::new("Thunder".to_string(), 110, 70, Type::Electric),
            Move::new("Psychic".to_string(), 90, 100, Type::Psychic),
            Move::new("Thunder Punch".to_string(), 75, 100, Type::Electric),
        ],
    );

    let typhlosion = Pokemon::new(
        "Typhlosion".to_string(),
        50,
        78,
        84,
        78,
        100,
        109,
        85,
        vec![Type::Fire],
        vec![
            Move::new("Flamethrower".to_string(), 90, 100, Type::Fire),
            Move::new("Fire Blast".to_string(), 110, 85, Type::Fire),
            Move::new("Eruption".to_string(), 150, 100, Type::Fire),
            Move::new("Overheat".to_string(), 140, 90, Type::Fire),
        ],
    );

    let dusknoir = Pokemon::new(
        "Dusknoir".to_string(),
        50,
        45,
        100,
        135,
        45,
        65,
        135,
        vec![Type::Ghost],
        vec![
            Move::new("Shadow Punch".to_string(), 60, 100, Type::Ghost),
            Move::new("Shadow Ball".to_string(), 80, 100, Type::Ghost),
            Move::new("Ice Punch".to_string(), 75, 100, Type::Ice),
            Move::new("Focus Blast".to_string(), 120, 70, Type::Fighting),
        ],
    );

    let ampharos = Pokemon::new(
        "Ampharos".to_string(),
        50,
        90,
        75,
        85,
        115,
        90,
        55,
        vec![Type::Electric],
        vec![
            Move::new("Thunderbolt".to_string(), 90, 100, Type::Electric),
            Move::new("Thunder".to_string(), 110, 70, Type::Electric),
            Move::new("Dragon Pulse".to_string(), 85, 100, Type::Dragon),
            Move::new("Thunder Punch".to_string(), 75, 100, Type::Electric),
        ],
    );

    let alolan_exeggutor = Pokemon::new(
        "Exeggutor-Alolan".to_string(),
        50,
        95,
        105,
        85,
        125,
        75,
        45,
        vec![Type::Grass, Type::Dragon],
        vec![
            Move::new("Dragon Pulse".to_string(), 85, 100, Type::Dragon),
            Move::new("Leaf Storm".to_string(), 130, 90, Type::Grass),
            Move::new("Psychic".to_string(), 90, 100, Type::Psychic),
            Move::new("Solar Beam".to_string(), 120, 100, Type::Grass),
        ],
    );

    let swampert = Pokemon::new(
        "Swampert".to_string(),
        50,
        100,
        110,
        90,
        85,
        90,
        60,
        vec![Type::Water, Type::Ground],
        vec![
            Move::new("Waterfall".to_string(), 80, 100, Type::Water),
            Move::new("Hydro Pump".to_string(), 110, 80, Type::Water),
            Move::new("Earthquake".to_string(), 100, 100, Type::Ground),
            Move::new("Ice Punch".to_string(), 75, 100, Type::Ice),
        ],
    );

    let mut nic = Trainer::new(
        "Nic".to_string(),
        vec![
            alolan_raichu,
            typhlosion,
            dusknoir,
            ampharos,
            alolan_exeggutor,
            swampert,
        ],
        8000000,
    );

    let mut ash = Trainer::new(
        "Ash".to_string(),
        vec![
            espeon,
            charizard,
            pikachu,
            totodile,
            breloom,
            snorlax
        ],
        10000000,
    );

    let mut nic_pc = PC::new(& mut nic);

    nic_pc.deposit();
    nic_pc.deposit();
    nic_pc.deposit();
    nic_pc.deposit();
    nic_pc.deposit();

    let mut nic_vs_ash = Battle::new(&ash, &nic);

    nic_vs_ash.start();



}
