mod genetic;
use genetic::population::{Population, Range};
use genetic::fitness::HasFitness;
use genetic::fitness::alternating_binary_fitness;
use genetic::fitness::parps_fitness;

fn main() {
    let population_size = 10;
    let genome_size = 10;
    let range = Range::new(0, 2);
    let population = Population::<u8>::new(population_size, genome_size, range);

    for individual in population.individuals {
        let fitness = individual.genome.fitness(&alternating_binary_fitness);
        println!("{:?} : {:?}", individual, fitness);
    }
}
