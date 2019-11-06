use crate::dot::Dot;
use crate::goal::Goal;
use ggez::{Context, GameResult};
use rand::Rng;

use crate::POPULATION_SIZE;

pub struct Population {
    pub dots: Vec<Dot>,

    fitness_sum: f32,
    pub gen: u32,

    best_dot: Dot,

    min_step: u32,
    goal: Goal,
}

impl Population {
    pub fn new() -> Population {
        Population {
            dots: vec![Default::default(); POPULATION_SIZE],
            fitness_sum: 0.0,
            gen: 1,
            best_dot: Default::default(),
            min_step: 1000,
            goal: Goal::new(),
        }
    }

    pub fn show(&mut self, ctx: &mut Context) -> GameResult {
        for i in 1..self.dots.len() {
            self.dots[i].show(ctx)?;
        }
        self.dots[0].show(ctx)?;
        Ok(())
    }

    pub fn update(&mut self, goal: &Goal) {
        for d in self.dots.iter_mut() {
            if d.brain.step > self.min_step {
                d.dead = true;
            } else {
                d.update(goal);
            }
        }
    }

    pub fn calculate_fitness(&mut self) {
        for d in self.dots.iter_mut() {
            d.calculate_fitness(&self.goal);
        }
    }

    pub fn all_dots_dead(&self) -> bool {
        for d in self.dots.iter() {
            if d.dead == false && d.reached_goal == false {
                return false;
            }
        }
        return true;
    }

    pub fn natural_selection(&mut self) {
        let mut new_dots = vec![Default::default(); POPULATION_SIZE];
        self.set_best_dot();
        self.calculate_fitness_sum();

        new_dots[0] = self.best_dot.clone().make_baby();
        new_dots[0].is_best = true;
        for i in 1..new_dots.len() {
            let parent = self.select_parent();
            new_dots[i] = parent.make_baby();
        }

        self.dots = new_dots;
        self.gen += 1;
    }

    pub fn calculate_fitness_sum(&mut self) {
        self.fitness_sum = 0.0;
        for d in self.dots.iter() {
            self.fitness_sum += d.fitness;
        }
    }

    pub fn select_parent(&mut self) -> Dot {
        let rand = rand::thread_rng().gen_range(0.0, &self.fitness_sum);
        let mut running_sum: f32 = 0.0;

        for d in self.dots.iter() {
            running_sum += d.fitness;
            if running_sum > rand {
                //dot = d.clone();
                return d.clone();
            }
        }
        return Default::default();
    }

    pub fn mutate_babies(&mut self) {
        for i in 1..self.dots.len() {
            self.dots[i].brain.mutate();
        }
    }

    pub fn set_best_dot(&mut self) {
        for d in self.dots.iter() {
            if d.fitness > self.best_dot.fitness {
                self.best_dot = d.clone();
            }
        }

        if self.best_dot.reached_goal == true {
            self.min_step = self.best_dot.brain.step;
        }
    }
}
