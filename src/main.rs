use ggez;
use ggez::event;
use ggez::graphics;
use ggez::conf;

mod brain;
mod dot;
mod goal;
mod obstacle;
mod population;

use goal::*;
use obstacle::*;
use population::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub const POPULATION_SIZE: usize = 500;

struct MainState {
    population: Population,
    goal: Goal,
    obstacle: Obstacle,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let s = MainState {
            population: Population::new(),
            goal: Goal::new(),
            obstacle: Obstacle::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.population.gen == 1 {
            self.population.mutate_babies();
            self.population.gen += 1;
        } else if self.population.all_dots_dead() == true {
            self.population.calculate_fitness();
            self.population.natural_selection();
            self.population.mutate_babies();
        }
        self.population.update(&self.goal);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.goal.show(ctx)?;
        self.obstacle.show(ctx)?;
        self.population.show(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let mut c = conf::Conf::new();
    c.window_mode.width = WINDOW_WIDTH;
    c.window_mode.height = WINDOW_HEIGHT;
    c.window_setup.title = String::from("Smart Dots");

    let cb = ggez::ContextBuilder::new("Smart Dots", "Yohann Delafollye").conf(c);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
