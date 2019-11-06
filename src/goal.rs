use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

use crate::WINDOW_WIDTH;

pub struct Goal {
    pub pos: Point2<f32>,
}

impl Goal {
    pub fn new() -> Goal {
        Goal {
            pos: Point2::new(WINDOW_WIDTH / 2.0 , 0.0),
        }
    }
    pub fn show(&mut self, ctx: &mut Context) -> GameResult {
        let sprite = graphics::Image::new(ctx, "/pikachu.png").unwrap();
        graphics::draw(ctx, &sprite, graphics::DrawParam::default().dest(self.pos))?;
        Ok(())
    }
}
