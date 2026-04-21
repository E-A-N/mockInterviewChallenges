/*
We're building a pokemon clone and need a POC for the evolution system.

A pokemon evolves during a level up event and their new level matches the next part of the evolution line level requirement.

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
*/


const TYPES = {
    fire: 0,
    water: 1,
    lightning: 2,
    grass: 3,
}

const STATS = {
    attack: 1,
    defense: 1,
    speed: 3
}

const POKEMON = {
    name: "pika",
    speciesIndex: 0,
    type: 0,
    level: 1,
    stats: null,
    evolutionLine: []
}

const main = () => {
	let poke1 = Object.assign({}, POKEMON);
  	poke1.name = "Pichu";
  	poke1.stats = Object.assign({}, STATS)
  	poke1.type = TYPES.lightning;
  	poke1.level = 1;
  	poke1.evolutionLine = [
      [0, "Pichu"],
      [16, "Pikachu"],
      [32, "Raichu"]
    ];

	let poke2 = Object.assign({}, POKEMON);
  	poke2.name = "Bulbasaur";
    poke2.stats = Object.assign({}, STATS)
  	poke2.type = TYPES.grass;
  	poke2.level = 15;
  	poke2.evolutionLine = [
      [0, "Bulbasaur"],
      [16, "Ivysaur"],
      [32, "Venusaur"]
    ];

	levelUp(poke1);
  	levelUp(poke2);

	console.log(poke1);
	console.log(poke2);
}

const levelUp = (subjectPokemon) => {
	if (subjectPokemon.level === 99){
		return      
    }
	subjectPokemon.level += 1;
	subjectPokemon.stats.attack += 1;
  	subjectPokemon.stats.defense += 1;
    subjectPokemon.stats.speed += 1;
  
  	const evolutionLine = subjectPokemon.evolutionLine;
  	const speciesIndex = subjectPokemon.speciesIndex;
  	
	const canEvolve = speciesIndex + 1 < evolutionLine.length
          && subjectPokemon.level >= evolutionLine[speciesIndex + 1][0];
  	if (canEvolve){
		evolution(subjectPokemon);
    }
}

const evolution = (subjectPokemon) => {
  	let oldSpecies = subjectPokemon.evolutionLine[subjectPokemon.speciesIndex];
    let newSpecies = subjectPokemon.evolutionLine[subjectPokemon.speciesIndex + 1];

  	if (subjectPokemon.name === oldSpecies[1]){
    	subjectPokemon.name = newSpecies[1];
    }
      
    subjectPokemon.speciesIndex += 1;
	subjectPokemon.stats.attack *= 0.5;
  	subjectPokemon.stats.defense *= 0.5;
    subjectPokemon.stats.speed *= 0.5;
}
main();
















