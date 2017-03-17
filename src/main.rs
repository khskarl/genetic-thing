mod genetic;
use genetic::population::{Population, Range};
use genetic::fitness::HasFitness;

fn main() {
    let population_size = 10;
    let genome_size = 10;
    let range = Range::new(0, 2);
    let population = Population::<u8>::new(population_size, genome_size, range);

    let memes = Vec::<u64>::new();
    for individual in population.individuals {
        let fitness = individual.genome.fitness();
        println!("{:?} : {:?}", individual, fitness);
    }

}
