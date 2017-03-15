mod population;
use population::{Population, Range};

#[derive(Debug)]
pub enum FitnessType {
    Integer(i32),
    Real(f32),
}

trait HasFitness {
    fn fitness(&self) -> FitnessType;
}

impl HasFitness for Vec<u8> {
    fn fitness(&self) -> FitnessType {
        let mut was_zero = self[0] == 0;
        let mut fitness = 0;

        for gene in self {
            let is_zero = *gene == 0;
            if was_zero != is_zero {
                fitness += 1;
            }
            was_zero = is_zero;
        }

        FitnessType::Integer(fitness)
    }
}

impl HasFitness for Vec<i32> {
    fn fitness(&self) -> FitnessType {
        let mut was_even = self[0] == 0;
        let mut fitness = 0;

        for gene in self {
            let is_even = *gene % 2 == 0;
            if was_even != is_even {
                fitness += 1;
            }
            was_even = is_even;
        }

        FitnessType::Integer(fitness)
    }
}

impl HasFitness for Vec<f32> {
    fn fitness(&self) -> FitnessType {
        let mut fitness = 0.0;

        for gene in self {
            fitness += gene * gene;
        }

        FitnessType::Real(fitness)
    }
}

fn main() {
    let population_size = 10;
    let genome_size = 10;
    let range = Range::new(0, 2);
    let population = Population::<u8>::new(population_size, genome_size, range);

    for individual in population.individuals {
        let fitness = individual.genome.fitness();
        println!("{:?} : {:?}", individual, fitness);
    }

}
