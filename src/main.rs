use ggez::{event, graphics, Context, GameResult};
use ggez::conf;
//use std::{env, path};


struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
    resize_projection: bool,
}

struct MainState {
    window_settings: WindowSettings,
    frames: f64,
    angle: f32
}


impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { frames: 0.0, angle: 0.0,
            window_settings: WindowSettings {
            toggle_fullscreen: true,
            is_fullscreen: true,
            resize_projection: false,
        } };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            self.angle += 0.01;

        if self.window_settings.toggle_fullscreen {
            let fullscreen_type = if self.window_settings.is_fullscreen {
                conf::FullscreenType::Desktop
            } else {
                conf::FullscreenType::Windowed
            };
            ctx.gfx.set_fullscreen(fullscreen_type)?;
            self.window_settings.toggle_fullscreen = true;
        }
    }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //let window = WindowMode;
        
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.9, 0.9, 0.3, 1.0]));
   
        // Text is drawn from the top-left corner.
        let offset = self.frames as f32;
        let dest_point = ggez::glam::Vec2::new(offset, offset);
        canvas.draw(
            graphics::Text::new("Endless Spire")
            // graphics::Text::new("press any button to start game")
                .set_scale(69.),
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