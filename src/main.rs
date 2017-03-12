mod population;
use population::{Population, Range};

fn main() {
    let population_size = 2;
    let genome_size = 4;
    let range = Range::new(0, 10);
    let mut population = Population::<u8, f32>::new(population_size, genome_size, range);

    //population.crossover(population.individuals[0], population.individuals[1]); 
}
