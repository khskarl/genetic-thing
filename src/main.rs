mod population;
use population::Population;
use population::Range;

// Everything else
fn main() {
    let population_size = 2;
    let genome_size = 4;
    let range = Range::new(0, 10);
    let mut population = Population::<u8>::new(population_size, genome_size, range);

    println!("{:?}", population.individuals[0]);
    println!("{:?}", population.individuals[1]);
    population.quick();
    println!("{:?}", population.individuals[0]);
    println!("{:?}", population.individuals[1]);

    //population.crossover(population.individuals[0], population.individuals[1]); 
}
