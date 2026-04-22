enum ElementTypes {
    Fire,
    Water,
    Lightning,
    Grass
}

struct Stats {
    attack: i32,
    defense: i32,
    speed: i32
}

struct Pokemon {
    name: String,
    species_index: i32,
    element_type: ElementTypes,
    level: i32,
    stats: Stats,
    evolution_line: [(i32, String); 3]
}

fn main() {
    println!("Hello, world!");
    let mut poke1 = Pokemon {
        name: String::from("Pichu"),
        species_index: 0,
        element_type: ElementTypes::Lightning,
        level: 0,
        stats: Stats {
            attack: 1,
            defense: 1,
            speed: 3,
        },
        evolution_line: [
            (0, String::from("Pichu")),
            (16, String::from("Pikachu")),
            (32, String::from("Raichu")),
        ]
    };
    poke1.level = 1;
    
    let mut poke2 = Pokemon {
        name: String::from("Bulbasaur"),
        species_index: 0,
        element_type: ElementTypes::Lightning,
        level: 0,
        stats: Stats {
            attack: 1,
            defense: 1,
            speed: 3,
        },
        evolution_line: [
            (0, String::from("Bulbasaur")),
            (16, String::from("Ivysaur")),
            (32, String::from("Venusaur")),
        ]
    };
    poke2.level = 2;
    
    level_up(&mut poke1);
}

fn level_up (subject_pokemon: &mut Pokemon) {
    if subject_pokemon.level >= 99 {
        return;
    }
    
    subject_pokemon.level += 1;
    subject_pokemon.stats.attack += 1;
    subject_pokemon.stats.defense += 1;
    subject_pokemon.stats.speed += 1;
    
    let species_index: usize = subject_pokemon.species_index as usize;
    let next_species_index: usize = species_index + 1;
    let evolution_line: &[(i32, String); 3] = &subject_pokemon.evolution_line;
    let can_evolve: bool = species_index + 1 < evolution_line.len()
        && subject_pokemon.level >= evolution_line[next_species_index].0;
}

fn evolution (subject_pokemon: &mut Pokemon){
    let oldSpecies: &(i32, String) = subect_pokemon.evolution_line[sub] 
}
