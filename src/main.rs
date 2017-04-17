extern crate gnuplot;
use gnuplot::{Figure, Color};
use gnuplot::PlotOption::LineWidth;

mod genetic;
use genetic::helpers::Range;
use genetic::population::{Population};
use genetic::fitness::{max_alternating_bits, min_dejong};
use genetic::crossover::{one_point_crossover, uniform_average_crossover, uniform_crossover};
use genetic::mutation::{bit_flip, delta_mutation, gaussian_mutation};

fn main() {
    let total_generations = 250;

    let population_size = 50;
    let genome_size = 2;
    let crossover_probability = 0.95;
    let mutation_probability = 0.05;
    let has_elitism = true;
    let fitness_function = min_dejong;
    let mutation_function = gaussian_mutation;
    let mut population = Population::<f32>::new(population_size,
                                                genome_size,
                                                crossover_probability,
                                                mutation_probability,
                                                Range::new(-3.0, 3.0),
                                                has_elitism,
                                                fitness_function,
                                                uniform_average_crossover,
                                                mutation_function);

    println!("Initial population");
    population.print();

    for current_generation in 0..total_generations {
        println!("\nA E S T H E T I C S: {}", current_generation);
        population.iterate_generation();
        population.print();
    }

    
    if let Some(best_individual) = population.best_individual_in_generation.last() {
        if let Some(best_fitness) = population.best_fitness_in_generation.last() {
            println!("Best genome: {:?} : {}", best_individual.genome, best_fitness);
        }
    }

    show_convergence_plot(&population.average_fitness_in_generation,
                          &population.best_fitness_in_generation);

}

fn show_convergence_plot(average_fitnesses: &Vec<f32>, best_fitnesses: &Vec<f32>) {
    let generations: Vec<usize> = (0..average_fitnesses.len()).collect();

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&generations,
               average_fitnesses,
               &[Color("#505050"), LineWidth(1.5)])
        .lines(&generations,
               best_fitnesses,
               &[Color("#0072bd"), LineWidth(1.5)]);
    fg.show();
}
