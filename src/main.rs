use ggez::{event, graphics, Context, GameResult};
//use std::{env, path};

struct MainState {
    frames: f64
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { frames: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.9, 0.9, 0.3, 1.0]));

        // Text is drawn from the top-left corner.
        let offset = self.frames as f32 / 5.0;
        let dest_point = ggez::glam::Vec2::new(offset, offset);
        canvas.draw(
            graphics::Text::new("Endless Spire")
                .set_scale(48.),
            dest_point,
        );

        canvas.finish(ctx)?;

        self.frames += 0.1;
        if (self.frames % 100.0) == 0.0 {
            println!("FPS: {}", ctx.time.fps());
        }

        Ok(())
    }
}


pub fn main() -> GameResult {
    
    let cb = ggez::ContextBuilder::new("Endless Spire", "me");//.add_resource_path(resource_dir);
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Endless Spire");

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}