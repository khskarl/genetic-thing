use std::f32;

extern crate gnuplot;
use gnuplot::{AxesCommon, Figure, Color};
use gnuplot::LabelOption::TextColor;
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
                       deceptive_f3,
                       deceptive_f3s,
                       deceptive_4};
use genetic::crossover::{one_point_crossover,
                         one_point_crossover_3,
                         uniform_average_crossover,
                         uniform_crossover,
                         uniform_crossover_3,
                         partially_matched_crossover};
use genetic::mutation::{bit_flip,
                        swap_position,
                        random_int,
                        delta_mutation,
                        gaussian_mutation};

fn main() {
    let total_generations = 10000;
    let population_size = 50;
    let genome_size = 30;

    let crossover_probability = 0.98;
    let mutation_probability = 0.001;
    
    let has_elitism = true;
    let has_scaling = true;
    let has_generation_gap = true;
    let has_fitness_sharing = true;
    let crowding_factor = 20;

    let distance_function = hamming_distance;
    
    let fitness_function = deceptive_f3s;
    let mutation_function = bit_flip;
    
    let mut population = Population::<u8>::new(population_size,
                                               genome_size,
                                               crossover_probability,
                                               mutation_probability,
                                               Range::new(0, 1),
                                               has_elitism,
                                               has_scaling,
                                               has_generation_gap,
                                               has_fitness_sharing,
                                               crowding_factor,
                                               distance_function,
                                               fitness_function,
                                               one_point_crossover,
                                               mutation_function);
    
    println!("Initial population");
    population.print();

    for current_generation in 0..total_generations {
        population.iterate_generation(current_generation, total_generations);
        println!("\nGeneration: {}", current_generation);
        //population.print();
    }

    population.print_best_individual_diagnostic();
    
    show_convergence_plot(&population.average_fitness_in_generation,
                          &population.best_fitness_in_generation);
    
    show_diversity_plot(&population.diversity_in_generation);
}

fn show_convergence_plot(average_fitnesses: &Vec<f32>, best_fitnesses: &Vec<f32>) {
    let generations: Vec<usize> = (0..average_fitnesses.len()).collect();

    let mut fg = Figure::new();
    fg.set_terminal("wxt size 800, 400", "");
    {
        let axes = fg.axes2d()
            .lines(&generations,
                   average_fitnesses,
                   &[Color("#505050"), LineWidth(1.2)])
            .lines(&generations,
                   best_fitnesses,
                   &[Color("#0072bd"), LineWidth(1.2)]);

        let label_options = &[gnuplot::LabelOption::TextColor("black")];
        axes.set_x_label("Gerações", label_options);
        axes.set_y_label("Fitness", label_options);
    }
    fg.show();
} 

fn show_diversity_plot(diversity_in_generations: &Vec<f32>) {
    let generations: Vec<usize> = (0..diversity_in_generations.len()).collect();

    let mut fg = Figure::new();
    fg.set_terminal("wxt size 800, 400", "");
    {
        let axes = fg.axes2d()
            .lines(&generations,
                   diversity_in_generations,
                   &[Color("#505050"), LineWidth(1.2)]);
    
        let label_options = &[gnuplot::LabelOption::TextColor("black")];
        axes.set_x_label("Gerações", label_options);
        axes.set_y_label("Diversidade", label_options);
    }
    fg.show();
}

