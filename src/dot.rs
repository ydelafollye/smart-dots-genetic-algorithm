use ggez::nalgebra;
use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

use crate::brain::Brain;
use crate::goal::Goal;

use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

#[derive(Clone)]
pub struct Dot {
    pub pos: Point2<f32>,
    pub acc: Point2<f32>,
    pub vel: Point2<f32>,
    pub brain: Brain,

    pub dead: bool,
    pub reached_goal: bool,
    pub is_best: bool,

    pub fitness: f32,
}

pub trait CustomPoint2 {
    fn add_points(&mut self, other: &Point2<f32>);
    fn from_angle(&mut self, angle: f32);
}

impl CustomPoint2 for Point2<f32> {
    fn add_points(&mut self, other: &Point2<f32>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    fn from_angle(&mut self, angle: f32) {
        self.x = angle.cos();
        self.y = angle.sin();
    }
}

impl Default for Dot {
    fn default() -> Dot {
        Dot {
            pos: Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT - 15.0),
            acc: Point2::new(0.0, 0.0),
            vel: Point2::new(0.0, 0.0),
            brain: Brain::new(500),
            dead: false,
            reached_goal: false,
            is_best: false,
            fitness: 0.0,
        }
    }
}

impl Dot {
    pub fn show(&mut self, ctx: &mut Context) -> GameResult {
        let pokeball = |&is_best| -> String {
            if is_best == true {
                "/masterball.png".to_string()
            } else {
                "/pokeball.png".to_string()
            }
        };
        let sprite = graphics::Image::new(ctx, pokeball(&self.is_best)).unwrap();
        graphics::draw(ctx, &sprite, graphics::DrawParam::default().dest(self.pos))?;
        Ok(())
    }

    pub fn stir(&mut self) {
        if self.brain.directions.len() > self.brain.step as usize {
            self.acc = self.brain.directions[self.brain.step as usize];
            self.brain.step += 1;
        } else {
            self.dead = true;
        }
        &self.vel.add_points(&self.acc);
        // TODO limit
        &self.pos.add_points(&self.vel);
    }

    pub fn update(&mut self, goal: &Goal) {
        if self.dead == false && self.reached_goal == false {
            self.stir();
            if self.pos.x <= 10.0
                || self.pos.y <= 10.0
                || self.pos.x >= WINDOW_WIDTH - 10.0
                || self.pos.y >= WINDOW_HEIGHT - 10.0
            {
                self.dead = true;
            } else if nalgebra::distance(&self.pos, &goal.pos) < 10.0 {
                self.reached_goal = true;
            } else if self.pos.x <= 600.0
                && self.pos.y <= 310.0
                && self.pos.x >= 0.0
                && self.pos.y >= 300.0
            {
                self.dead = true;
            }
        }
    }

    pub fn calculate_fitness(&mut self, goal: &Goal) {
        if self.reached_goal == true {
            self.fitness = 1.0 / 16.0 + 10000.0 / (self.brain.step as f32 * self.brain.step as f32);
        } else {
            let distance_to_goal: f32 = nalgebra::distance(&self.pos, &goal.pos);
            self.fitness = 1.0 / (distance_to_goal * distance_to_goal);
        }
    }

    pub fn make_baby(self) -> Dot {
        Dot {
            brain: self.brain.clone_brain(),
            ..Default::default()
        }
    }
}
