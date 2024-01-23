use crate::simulation::{Simulation, STOCKS};

mod individual;
mod simulation;
//CONSTANTS

fn main() {
    //create simulation
    let mut sim = Simulation::new();
    //get the best solution
    let best_solution = sim.run();
    println!("Best Individual in the population:");
    for i in 0..STOCKS.len() {
        println!("{:?}", best_solution.chromosome[i]);
    }
    println!("fitness: {}", best_solution.fitness);
}
