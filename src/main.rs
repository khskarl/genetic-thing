mod genetic;
use genetic::population::{Population, Range};
use genetic::fitness::max_alternating_bits;
use genetic::crossover::one_point_crossover;
use genetic::mutation::bit_flip;

fn main() {
    let population_size = 4;
    let genome_size = 10;
    let crossover_probability = 0.95;
    let mutation_probability = 0.05;
    let mut population = Population::<u8>::new(population_size,
                                               genome_size,
                                               crossover_probability,
                                               mutation_probability,
                                               Range::new(0, 2),
                                               max_alternating_bits,
                                               one_point_crossover,
                                               bit_flip);

    population.iterate_generation();

    for i in 0..population.individuals.len() {
        {
            let individual = &population.individuals[i];
            let fitness = population.fitnesses[i];
            println!("{:?} : {}", individual, fitness);
        }

        //&population.individuals[i].genome.mutate(&bit_flip, 0.1);
    }

    population.crossover(0, 1);

    population.iterate_generation();
    println!("A E S T H E T I C S");

    for i in 0..population.individuals.len() {
        let individual = &population.individuals[i];
        let fitness = population.fitnesses[i];
        println!("{:?} : {}", individual, fitness);
    }
}
