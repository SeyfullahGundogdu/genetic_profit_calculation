use rand::{seq::SliceRandom, thread_rng};

use crate::individual::Individual;

//constants
const P_SIZE: usize = 20; //population size
const GEN_LIMIT: i32 = 5000; //generation limit 
const WEAK_CROSS_RATE: f32 = 0.002; //rate of weak individuals getting picked
pub const STOCKS: [i32; 5] = [30, 40, 20, 40, 20];
pub const MUTATION_RATE: f32 = 0.99;
pub const CITY_PRICES: [[i32; STOCKS.len()]; 5] = [
    [1, 4, 6, 4, 4],
    [3, 8, 2, 5, 15],
    [3, 12, 3, 5, 5],
    [2, 6, 10, 2, 4],
    [10, 5, 12, 6, 3],
];

//simulation has a list of individuals
pub struct Simulation {
    population: Vec<Individual>,
}

//functions that Simulation struct and/or instances implement
impl Simulation {
    //crossover the entire population
    fn crossover_population(&self) -> Vec<Individual> {
        let mut rng = thread_rng();
        let mut next_gen: Vec<Individual> = vec![];
        //calculate total fitness
        let total_fitness: f32 = self.population.iter().map(|x| x.fitness).sum();
        //pick the best individual and carry it over to the next gen.
        let best_index = self.best_indv();
        next_gen.push(self.population[best_index]);

        //pick 2 parents using weighted probability and crossover
        for _ in 0..self.population.len() - 1 {
            // pick 2 parents
            let parents = self
                .population
                .choose_multiple_weighted(&mut rng, 2, |item| item.fitness / total_fitness)
                .unwrap()
                .collect::<Vec<&Individual>>();
            //push their child to the next gen
            next_gen.push(parents[0].crossover(parents[1]));
            if parents.contains(&&self.population[best_index]) {
                continue;
            }
        }
        next_gen
    }

    //find the indice of best individual in a population
    fn best_indv(&self) -> usize {
        let mut best_index = 0;
        for i in 0..self.population.len() {
            if self.population[i].fitness > self.population[best_index].fitness {
                best_index = i;
            }
        }
        best_index
    }

    //create new simulation instance
    pub fn new() -> Self {
        Simulation { population: vec![] }
    }
    //run simulation
    pub fn run(&mut self) -> Individual {
        //next generation for simulation
        let mut next_pop: Vec<Individual> = vec![];
        //default individual that we will use as the min value
        let mut best_indv = Individual::default();
        //clear the population and create new random population
        self.population.clear();
        for _ in 0..P_SIZE {
            self.population.push(Individual::new());
        }
        //sort the population
        self.population
            .sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        //generations
        for _ in 0..GEN_LIMIT {
            let mut crossed_pop = self.crossover_population();
            //sort the population depending on the fitness(from largest to smallest)
            crossed_pop.sort_unstable_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());

            for (j, child) in crossed_pop.iter_mut().enumerate().take(P_SIZE) {
                let curr = self.population[j];
                child.generation += 1;

                // compare current individual with child with the same index
                //if child is better, put the child in the next population
                //if child is worse, put the child within a probability
                if child.fitness > curr.fitness
                    || rand::Rng::gen::<f32>(&mut rand::thread_rng()) < WEAK_CROSS_RATE
                {
                    next_pop.push(*child);
                }
                //if child is worse and couldn't be picked, put the parent to the next generation
                else {
                    next_pop.push(curr);
                }
            }
        }
        //copy over the next population
        self.population = next_pop.clone();
        //sort the population after getting new childs
        self.population
            .sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());

        //get the best candidate and compare it to our current best individual
        //save better one
        let best_candidate = self.population[self.best_indv()];
        if best_indv.fitness <= best_candidate.fitness {
            best_indv = best_candidate.to_owned();
        }
        best_indv
    }
}
