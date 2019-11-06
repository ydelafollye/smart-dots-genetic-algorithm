use core::f32::consts::PI;
use ggez::nalgebra::Point2;
use rand::Rng;

use crate::dot::CustomPoint2;

#[derive(Clone)]
pub struct Brain {
    pub directions: Vec<Point2<f32>>,
    pub step: u32,
}

impl Brain {
    pub fn new(size: usize) -> Brain {
        let mut brain = Brain {
            directions: vec![Point2::new(0.0, 0.0); size],
            step: 0,
        };
        brain.randomize();
        return brain;
    }

    pub fn clone_brain(self) -> Brain {
        Brain {
            directions: self.directions,
            step: 0,
        }
    }

    fn randomize(&mut self) {
        for d in self.directions.iter_mut() {
            let random_angle: f32 = rand::thread_rng().gen_range(0.0, 2.0 * PI);
            d.from_angle(random_angle);
        }
    }

    pub fn mutate(&mut self) {
        let mutation_rate: f32 = 0.01;
        for d in self.directions.iter_mut() {
            let rand = rand::thread_rng().gen_range(0.0, 1.0);
            if rand < mutation_rate {
                let random_angle: f32 = rand::thread_rng().gen_range(0.0, 2.0 * PI);
                d.from_angle(random_angle);
            }
        }
    }
}
