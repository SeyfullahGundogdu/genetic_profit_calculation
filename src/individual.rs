use std::cmp;

use rand::{thread_rng, Rng};

use crate::simulation::{CITY_PRICES, MUTATION_RATE, STOCKS};

#[derive(Clone, Copy, PartialEq)]
pub struct Individual {
    pub chromosome: [[i32; CITY_PRICES.len()]; STOCKS.len()],
    pub fitness: f32,
    pub generation: u32,
}

impl Individual {
    // crossover an Individual with another one
    // copy the first parent and change its chromosome by
    // choosing a random indice and overwriting through the
    // end of the vector by using other parent
    pub fn crossover(&self, other: &Self) -> Self {
        //copy first parent into child
        let mut child: Individual = self.to_owned();
        // helper method randomness
        let mut rng = rand::thread_rng();
        let crossover_index = rng.gen_range(0..STOCKS.len());
        //overwrite the randomly chosen interval
        for i in crossover_index..STOCKS.len() {
            child.chromosome[i] = other.chromosome[i];
        }
        //chance of mutating when creating new child
        if rng.gen::<f32>() <= MUTATION_RATE {
            child.mutate();
        }
        // increment generation and calculate fitness, then return child
        child.generation += 1;
        child.fitness = child.fitness();
        child
    }

    //calculate base fitness filtered by a city
    fn f_base_city(&self, city: usize) -> i32 {
        let mut income = 0;
        for (i, _) in CITY_PRICES.iter().enumerate() {
            income += self.chromosome[i][city] * CITY_PRICES[i][city]
        }
        income
    }

    //total base fitness
    fn f_base(&self) -> f32 {
        let mut total_income = 0;
        for i in 0..CITY_PRICES.len() {
            total_income += self.f_base_city(i);
        }
        total_income as f32
    }

    //calculate f1
    fn f1(&self) -> f32 {
        let mut visited_cities = 0;
        for i in 0..CITY_PRICES.len() {
            for j in 0..STOCKS.len() {
                if self.chromosome[j][i] == 0 {
                    continue;
                }
                visited_cities += 1;
            }
        }
        if visited_cities == CITY_PRICES.len() {
            100.0
        } else {
            0.0
        }
    }

    //calculate f2
    fn f2(&self) -> f32 {
        let mut income = 0.;
        for city in 0..CITY_PRICES.len() {
            let mut min: i32 = STOCKS.iter().max().unwrap().to_owned();
            let mut max = 0;

            for stock in 0..STOCKS.len() {
                max = cmp::max(self.chromosome[stock][city], max);
                min = cmp::min(self.chromosome[stock][city], min);
            }
            income += (self.f_base_city(city) * cmp::max(20 - (max - min), 0)) as f32 / 100f32;
        }
        income
    }

    //calculate f3
    fn f3(&self) -> f32 {
        let mut total_sold_stocks: Vec<i32> = vec![];
        for city in 0..CITY_PRICES.len() {
            let mut sold_stocks = 0;
            for stock in 0..STOCKS.len() {
                sold_stocks += self.chromosome[stock][city]
            }
            total_sold_stocks.push(sold_stocks);
        }
        let max = total_sold_stocks.iter().max().unwrap();
        let min = total_sold_stocks.iter().min().unwrap();
        (self.f_base() * cmp::max(20 - max + min, 0) as f32) / 100f32
    }

    //calculate total fitness
    fn fitness(&self) -> f32 {
        if !self.chromosome.iter().flatten().any(|&i| i == -1) {
            return self.f_base() + self.f1() + self.f2() + self.f3();
        }
        0.
    }

    //default function to create an Individual instance
    pub fn default() -> Self {
        Individual {
            chromosome: [[-1; CITY_PRICES.len()]; STOCKS.len()],
            fitness: 0.,
            generation: 0,
        }
    }

    //create a new Individual instance
    pub fn new() -> Self {
        let mut ind = Individual::default();
        let mut stocks = STOCKS;

        //choose random stocks to sell in cities, put the remaining stocks to the final city
        for (stock, count) in stocks.iter_mut().enumerate().take(STOCKS.len()) {
            for city in 0..CITY_PRICES.len() - 1 {
                let mut rng = thread_rng();
                ind.chromosome[stock][city] = rng.gen_range(0..*count);
                *count -= ind.chromosome[stock][city]
            }
            ind.chromosome[stock][CITY_PRICES.len() - 1] = *count;
            *count -= ind.chromosome[stock][CITY_PRICES.len() - 1]
        }
        //calculate fitness and return new instance of Individual
        ind.fitness = ind.fitness();
        ind
    }

    fn mutate(&mut self) {
        for stock in 0..STOCKS.len() {
            //choose 2 cities randomly, swap some of the stock between the two of them.
            let mut rng = rand::thread_rng();

            //loop forever until 2 cities are different
            loop {
                //create 2 random numbers
                let city_1 = rng.gen_range(0..CITY_PRICES.len());
                let city_2 = rng.gen_range(0..CITY_PRICES.len());

                //different cities chosen
                if city_1 != city_2 {
                    //if total stock in 2 cities are 0, return
                    let total_stock =
                        self.chromosome[stock][city_1] + self.chromosome[stock][city_2];
                    if total_stock == 0 {
                        break;
                    }
                    //else, pick some random number, take it from one city and add it to the other one
                    let stock_swap = rng.gen_range(0..=self.chromosome[stock][city_2]);
                    self.chromosome[stock][city_2] -= stock_swap;
                    self.chromosome[stock][city_1] += stock_swap;
                    break;
                }
            }
        }
    }
}
