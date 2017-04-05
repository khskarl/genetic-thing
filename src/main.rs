mod genetic;
use genetic::population::{Population, Range};
use genetic::fitness::HasFitness;
use genetic::fitness::max_alternating_bits;

fn main() {
    let population_size = 4;
    let genome_size = 10;
    let range = Range::new(0, 2);
    let mut population = Population::<u8>::new(population_size,
                                               genome_size,
                                               range,
                                               max_alternating_bits);

    population.iterate_generation();
    for i in 0..population.individuals.len() {
        let individual = &population.individuals[i];
        let fitness = population.fitnesses[i];
        println!("{:?} : {}", individual, fitness);
    }
}
