mod population;
use population::Population;
use population::Range;

// Everything else
fn main() {
    let population_size = 100;
    let genome_size = 10;
    let range = Range::new(0, 2);
    let population = Population::<u8>::new(population_size, genome_size, range);

    println!("{:?}", population.individuals[0]);
    println!("Length: {}", population.genome_size());
}
