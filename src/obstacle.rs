use ggez::{graphics, Context, GameResult};

pub struct Obstacle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Obstacle {
    pub fn new() -> Obstacle {
        Obstacle {
            x: 0.0,
            y: 300.0,
            w: 600.0,
            h: 10.0,
        }
    }

    pub fn show(&mut self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.x, self.y, self.w, self.h),
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
        Ok(())
    }
}
