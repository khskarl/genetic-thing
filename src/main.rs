extern crate gnuplot;
use gnuplot::{Figure, Caption, Color};

mod genetic;
use genetic::population::{Population, Range};
use genetic::fitness::max_alternating_bits;
use genetic::crossover::one_point_crossover;
use genetic::mutation::bit_flip;

fn main() {
    let total_generations = 100;

    let population_size = 5;
    let genome_size = 10;
    let crossover_probability = 0.95;
    let mutation_probability = 0.05;
    let has_elitism = true;
    let fitness_function = max_alternating_bits;
    let mutation_function = bit_flip;
    let mut population = Population::<u8>::new(population_size,
                                               genome_size,
                                               crossover_probability,
                                               mutation_probability,
                                               Range::new(0, 2),
                                               has_elitism,
                                               fitness_function,
                                               one_point_crossover,
                                               mutation_function);

    for current_generation in 0..total_generations {
        println!("A E S T H E T I C S: {}", current_generation);

        for i in 0..population.individuals.len() {
            let individual = &population.individuals[i];
            let fitness = population.fitnesses[i];
            println!("{:?} : {}", individual, fitness);
        }

        population.iterate_generation();
    }

    // Convergence plot
    show_convergence_plot(population.average_fitness_in_generation,
                          population.best_fitness_in_generation);
}

fn show_convergence_plot(average_fitnesses: Vec<f32>,
                         best_fitnesses: Vec<f32>) {
    let generations: Vec<usize> = (0..average_fitnesses.len()).collect();
    
    let mut fg = Figure::new();
    fg.axes2d().lines(&generations, &average_fitnesses, &[Color("black")]); 
    fg.axes2d().lines(&generations, &best_fitnesses, &[Color("red")]);
    fg.show();
}
