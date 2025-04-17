use rand::{seq::IndexedRandom, Rng};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::Game;

use super::Config;

#[test]
fn genetic_algorithm() {
    use std::{fs::File, io::Read};
    let mut rng = rand::rng();

    let mut population = Population::new_random(500, 200, &mut rng);

    let positions = {
        let mut positions = String::new();
        let mut f = File::open("puzzles_use.txt").unwrap();
        f.read_to_string(&mut positions).unwrap();

        positions
            .lines()
            .map(|fen| Game::from_fen(fen).unwrap())
            .collect::<Vec<_>>()
    };

    population.evaluate(&positions);

    for _ in 0..10 {
        population.new_generation(&positions, &mut rng);
    }

    let best = population.best_individual.unwrap();
}

const CROSSOVER_RATE: f64 = 0.7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Individual {
    evaluation: Option<usize>,
    chromosome: Config,
}

impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.evaluation
            .unwrap()
            .partial_cmp(&other.evaluation.unwrap())
    }
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluation.unwrap().cmp(&other.evaluation.unwrap())
    }
}

pub struct Population {
    population_size: usize,
    parent_size: usize,
    individuals: Vec<Individual>,
    best_individual: Option<Individual>,
    found_generation: usize,
    current_generation: usize,
}

impl Population {
    pub fn new_random(population_size: usize, parent_size: usize, rng: &mut impl Rng) -> Self {
        let individuals = (0..population_size)
            .map(|_| Individual {
                evaluation: None,
                chromosome: Config::new_random(rng),
            })
            .collect();

        Self {
            population_size,
            parent_size,
            individuals,
            best_individual: None,
            found_generation: 0,
            current_generation: 0,
        }
    }

    pub fn choose_parent(&mut self, parents: &mut Vec<Config>, rng: &mut impl Rng) {
        let selected = self.individuals.choose_multiple(rng, 2);

        parents.push(selected.min().unwrap().chromosome.clone())
    }

    pub fn evaluate(&mut self, positions: &[Game]) {
        // let mut best_individual = self.best_individual.take();

        self.individuals
            .par_iter_mut()
            .for_each(|i| i.evaluation = Some(i.chromosome.evaluate(4, positions)));

        let best = self
            .individuals
            .iter()
            .min()
            .expect("No individuals in population");

        if Some(best) > self.best_individual.as_ref() {
            self.best_individual = Some(best.clone());
            self.found_generation = self.current_generation;
        }
    }

    pub fn new_generation(&mut self, positions: &[Game], rng: &mut impl Rng) {
        let mut parents = Vec::new();
        while parents.len() < self.parent_size {
            self.choose_parent(&mut parents, rng);
        }

        let mut new_individuals = Vec::with_capacity(self.population_size);

        while new_individuals.len() < self.population_size {
            let mut p = parents.choose_multiple(rng, 2);

            let (mut child_0, mut child_1) = if rand::random_bool(CROSSOVER_RATE) {
                p.next().unwrap().crossover(p.next().unwrap(), rng)
            } else {
                (p.next().unwrap().clone(), p.next().unwrap().clone())
            };

            child_0.mutate(rng);
            child_1.mutate(rng);

            new_individuals.push(Individual {
                evaluation: None,
                chromosome: child_0,
            });
            new_individuals.push(Individual {
                evaluation: None,
                chromosome: child_1,
            });
        }

        self.individuals = new_individuals;
        self.evaluate(positions);
        self.current_generation += 1;
    }
}
