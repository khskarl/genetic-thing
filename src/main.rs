extern crate gnuplot;
use gnuplot::{Figure, Color};
use gnuplot::PlotOption::LineWidth;

mod genetic;
use genetic::helpers::Range;
use genetic::helpers::{hamming_distance, euclidean_distance_int, euclidean_distance_float};
use genetic::population::{Population};
use genetic::fitness::{max_alternating_bits,
                       max_alternating_even_odd,
                       pattern_recognition,
                       min_dejong,
                       n_queens,
                       path_fitness,
                       fully_deceptive_f3};
use genetic::crossover::{one_point_crossover,
                         uniform_average_crossover,
                         uniform_crossover,
                         partially_matched_crossover};
use genetic::mutation::{bit_flip,
                        swap_position,
                        random_int,
                        delta_mutation,
                        gaussian_mutation};

fn main() {
    let total_generations = 100000;

    let population_size = 25;
    let genome_size = 100;
    let crossover_probability = 0.95;
    let mutation_probability = 0.05;
    let has_elitism = true;
    let fitness_function = path_fitness;
    let mutation_function = random_int;
    let mut population = Population::<i32>::new(population_size,
                                                genome_size,
                                                crossover_probability,
                                                mutation_probability,
                                                Range::new(0, 2),
                                                has_elitism,
                                                euclidean_distance_int,
                                                fitness_function,
                                                one_point_crossover,
                                                mutation_function);
    
    println!("Initial population");
    population.print();

    for current_generation in 0..total_generations {
        println!("\nA E S T H E T I C S: {}", current_generation);
        population.iterate_generation(current_generation, total_generations);
        //population.print();
    }

    // let gene = vec![1, 1, 3, 3, 1, 3, 2, 2, 0, 0];
    // println!("{:?}", gene);
    // (fitness_function)(&gene, &Range::new(0, 3));
    
    population.print_best_individual_diagnostic();
    
    show_convergence_plot(&population.average_fitness_in_generation,
                          &population.best_fitness_in_generation);
    
    //show_diversity_plot(&population.diversity_in_generation);
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

fn show_diversity_plot(diversity_in_generations: &Vec<f32>) {
    let generations: Vec<usize> = (0..diversity_in_generations.len()).collect();

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&generations,
               diversity_in_generations,
               &[Color("#505050"), LineWidth(1.5)]);
    fg.show();
}
