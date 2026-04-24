```swift
/*
We're building a pokemon clone and need a POC for the evolution system.
For now, we only need a headless solution of pure game logic.

A pokemon evolves during a level up event and their new level matches the next 
tier of the evolution line level requirement.
Evolving a pokemon should entail the following:
	- Increase in stats
	- Change in species
	- If the pokemon's name is the same as its species, change its name to the name of the species


Write a level-up function that does the following:
	- Apply a stat and level increase
	- Evolve the pokemon if the level matches the minimum requirement in the next evolution line

Bonus:
	- Implement items and evolution stones
	- Logic/Architecture for cancelling evolutions
	- Delta mathematics
	- Latency/UX considerations
	- Branching Evolutions (IE Eevee)
*/

// Enum for Pokémon types
enum Type {
    case fire
    case grass
    case water
    case lightning
}

// Struct to represent Pokémon stats
struct Stats {
    var attack: Int
    var defense: Int
    var speed: Int
}

// Struct to represent evolution stages
struct EvolutionStage {
    var requiredLevel: Int
    var speciesName: String // Name of the species
}

// Struct to represent a Pokémon
struct Pokemon {
    var name: String           // Nickname of the pokemon
    var speciesIndex: Int      // Index in evolution line
    var speciesName: String 	// Species of the pokemon
    var type: Type             // Type of the Pokémon
    var level: Int             // Current level
    var stats: Stats           // Pokémon stats
    var array evolutionLine: [EvolutionStage]  // Array of evolution stages 
}
```
