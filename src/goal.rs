use ggez::nalgebra::Point2;
use ggez::{graphics, Context, GameResult};

pub struct Goal {
    pub pos: Point2<f32>,
}

impl Goal {
    pub fn new() -> Goal {
        Goal {
            pos: Point2::new(400.0, 10.0),
        }
    }
    pub fn show(&mut self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            5.0,
            0.1,
            graphics::Color::new(255.0, 0.0, 0.0, 255.0),
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        Ok(())
    }
}
