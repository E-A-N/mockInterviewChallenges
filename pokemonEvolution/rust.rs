enum ElementTypes {
    Fire,
    Water,
    Lightning,
    Grass
}

struct Stats {
    attack: f32,
    defense: f32,
    speed: f32,
}

struct Pokemon {
    name: &'static str,
    species_index: usize,
    element_type: ElementTypes,
    level: i32,
    stats: Stats,
    evolution_line: [(i32, &'static str); 3]
}

fn main() {
    let mut poke1 = Pokemon {
        name: "Pichu",
        species_index: 0,
        element_type: ElementTypes::Lightning,
        level: 0,
        stats: Stats {
            attack: 1.0,
            defense: 1.0,
            speed: 3.0,
        },
        evolution_line: [
            (0, "Pichu"),
            (16, "Pikachu"),
            (32, "Raichu"),
        ]
    };
    poke1.level = 1;
    
    let mut poke2 = Pokemon {
        name: "Bulbasaur",
        species_index: 0,
        element_type: ElementTypes::Lightning,
        level: 0,
        stats: Stats {
            attack: 1.0,
            defense: 1.0,
            speed: 3.0,
        },
        evolution_line: [
            (0, "Bulbasaur"),
            (16, "Ivysaur"),
            (32, "Venusaur"),
        ]
    };
    poke2.level = 15;
    
    level_up(&mut poke1);
    level_up(&mut poke2);
}

fn level_up (subject_pokemon: &mut Pokemon) {
    if subject_pokemon.level >= 99 {
        return;
    }
    
    subject_pokemon.level += 1;
    subject_pokemon.stats.attack += 1.0;
    subject_pokemon.stats.defense += 1.0;
    subject_pokemon.stats.speed += 1.0;
    
    println!("{} has leveled up to: {}!", subject_pokemon.name, subject_pokemon.level);
    
    let species_index: usize = subject_pokemon.species_index;
    let next_species_index: usize = species_index + 1;
    let evolution_line: &[(i32, &'static str); 3] = &subject_pokemon.evolution_line;
    let can_evolve: bool = species_index + 1 < evolution_line.len()
        && subject_pokemon.level >= evolution_line[next_species_index].0;
    if can_evolve {
        evolution(subject_pokemon);
    }
}

fn evolution (subject_pokemon: &mut Pokemon){
    let species_index: usize = subject_pokemon.species_index;
    let next_species_index: usize = species_index + 1;

    let old_species: &'static str = subject_pokemon.evolution_line[species_index].1;
    let next_species: &'static str = subject_pokemon.evolution_line[next_species_index].1;
    if subject_pokemon.name == old_species {
        subject_pokemon.name = next_species;
    }

    subject_pokemon.species_index += 1;
    subject_pokemon.stats.attack *= 0.5;
    subject_pokemon.stats.defense *= 0.5;
    subject_pokemon.stats.speed *= 0.5;
    
    println!("{} evolved into: {}", old_species, next_species);
}
